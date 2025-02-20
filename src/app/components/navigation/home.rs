use gettextrs::*;
use gtk::prelude::*;

use crate::app::components::{Component, EventListener, ScreenFactory};
use crate::app::AppEvent;

fn find_listbox_descendant(widget: &gtk::Widget) -> Option<gtk::ListBox> {
    match widget.downcast_ref::<gtk::ListBox>() {
        Some(listbox) => Some(listbox.clone()),
        None => {
            let next = widget.first_child()?;
            find_listbox_descendant(&next)
        }
    }
}

pub struct HomePane {
    stack: gtk::Stack,
    stack_sidebar: gtk::StackSidebar,
    components: Vec<Box<dyn EventListener>>,
}

impl HomePane {
    pub fn new(stack_sidebar: gtk::StackSidebar, screen_factory: &ScreenFactory) -> Self {
        let library = screen_factory.make_library();
        let saved_playlists = screen_factory.make_saved_playlists();
        let saved_tracks = screen_factory.make_saved_tracks();
        let now_playing = screen_factory.make_now_playing();

        let stack = gtk::Stack::new();
        stack.set_transition_type(gtk::StackTransitionType::Crossfade);
        stack.add_titled(
            library.get_root_widget(),
            Some("library"),
            // translators: This is a sidebar entry to browse to saved albums.
            &gettext("Library"),
        );
        stack.add_titled(
            saved_playlists.get_root_widget(),
            Some("saved_playlists"),
            // translators: This is a sidebar entry to browse to saved playlists.
            &gettext("Playlists"),
        );
        stack.add_titled(
            saved_tracks.get_root_widget(),
            Some("saved_tracks"),
            // translators: This is a sidebar entry to browse to saved tracks.
            &gettext("Saved tracks"),
        );
        stack.add_titled(
            now_playing.get_root_widget(),
            Some("now_playing"),
            &gettext("Now playing"),
        );

        stack_sidebar.set_stack(&stack);

        Self {
            stack,
            stack_sidebar,
            components: vec![
                Box::new(library),
                Box::new(saved_playlists),
                Box::new(saved_tracks),
                Box::new(now_playing),
            ],
        }
    }

    pub fn connect_navigated<F: Fn() + 'static>(&self, f: F) {
        // stack sidebar wraps a listbox with a scroll window, so i'm cheating a bit there to get the listbox ;)
        if let Some(listbox) = find_listbox_descendant(self.stack_sidebar.upcast_ref()) {
            listbox.connect_row_activated(move |_, _| {
                f();
            });
        }
    }
}

impl Component for HomePane {
    fn get_root_widget(&self) -> &gtk::Widget {
        self.stack.upcast_ref()
    }

    fn get_children(&mut self) -> Option<&mut Vec<Box<dyn EventListener>>> {
        Some(&mut self.components)
    }
}

impl EventListener for HomePane {
    fn on_event(&mut self, event: &AppEvent) {
        if let AppEvent::NowPlayingShown = event {
            self.stack.set_visible_child_name("now_playing");
        }
        self.broadcast_event(event);
    }
}
