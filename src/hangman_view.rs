use cursive::{
    event::{Event, EventResult},
    traits::{Finder, Nameable},
    view::{Selector, ViewWrapper},
    views::{Canvas, LinearLayout, Panel, TextView},
    wrap_impl, View,
};

use crate::hangman::Hangman;
pub struct HangmanView<T: View> {
    hangman: Hangman,
    view: T,
}

impl HangmanView<Panel<LinearLayout>> {
    pub fn new() -> Self {
        let hangman = Hangman::new(&"hello".to_owned());

        let view = Panel::new(
            LinearLayout::vertical()
                .child(TextView::new(hangman.get_pic()).center().with_name("pic"))
                .child(Canvas::new(()))
                .child(
                    TextView::new(hangman.get_guessed_word())
                        .center()
                        .with_name("guessed_word"),
                )
                .child(Canvas::new(()))
                .child(TextView::new("Press a key to guess a letter").center()),
        );

        Self { hangman, view }
    }
}

impl<T: View> ViewWrapper for HangmanView<T> {
    wrap_impl!(self.view: T);

    fn wrap_on_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::Char(c) => {
                self.hangman.guess_letter(c);

                let pic = self.hangman.get_pic().to_owned();
                let guessed_word = self.hangman.get_guessed_word().to_owned();

                self.call_on(&Selector::Name("pic"), |v: &mut TextView| {
                    v.set_content(pic);
                });

                self.call_on(&Selector::Name("guessed_word"), |v: &mut TextView| {
                    v.set_content(guessed_word);
                });
            }
            _ => (),
        }

        EventResult::Consumed(None)
    }
}
