use ratatui::widgets::ListState;

#[derive(Clone, Debug)]
pub struct Model {
    pub items: Vec<TodoItem>,
    pub state: ListState,
}

#[derive(Clone, Debug)]
pub struct TodoItem {
    pub todo: String,
    pub info: String,
    pub status: Status,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Status {
    Todo,
    Completed,
}

impl Default for Model {
    fn default() -> Self {
        Model::from_iter([
            (
                Status::Todo,
                "Rewrite everything with Rust!",
                "I can't hold my inner voice. He tells me to rewrite the complete universe with Rust",
            ),
            (
                Status::Completed,
                "Rewrite all of your tui apps with Ratatui",
                "Yes, you heard that right. Go and replace your tui with Ratatui.",
            ),
            (
                Status::Todo,
                "Pet your cat",
                "Minnak loves to be pet by you! Don't forget to pet and give some treats!",
            ),
            (
                Status::Todo,
                "Walk with your dog",
                "Max is bored, go walk with him!",
            ),
            (
                Status::Completed,
                "Pay the bills",
                "Pay the train subscription!!!",
            ),
            (
                Status::Completed,
                "Refactor list example",
                "If you see this info that means I completed this task!",
            ),
            (
                Status::Todo,
                "Implement a neural network in pure Rust",
                "Because why use Python when you can have zero-cost abstractions and borrow checker headaches?",
            ),
            (
                Status::Todo,
                "Buy more coffee",
                "The compile times are getting longer, or maybe I'm just getting slower. Either way, caffeine is required.",
            ),
            (
                Status::Completed,
                "Fix that one annoying lifetime error",
                "It only took three days and a sacrifice to the crab god, but it finally compiles!",
            ),
            (
                Status::Todo,
                "Clean the mechanical keyboard",
                "There is a non-zero amount of crumbs between the blue switches. It’s affecting my WPM.",
            ),
            (
                Status::Todo,
                "Read the 'Rustonomicon'",
                "Into the dark depths of unsafe Rust we go. May the pointer aliasing rules have mercy.",
            ),
            (
                Status::Completed,
                "Hydrate",
                "Drank a full glass of water. Look at me, practicing self-care while writing low-level code.",
            ),
            (
                Status::Todo,
                "Argue about memory safety on the internet",
                "Someone said C is 'fine.' I must cordially explain why they are mistaken.",
            ),
            (
                Status::Todo,
                "Organize the .dotfiles",
                "Spend 4 hours configuring Neovim instead of actually working on the project.",
            ),
            (
                Status::Completed,
                "Update dependencies",
                "Ran 'cargo update'. Only 14 breaking changes in the ecosystem today. A new record!",
            ),
            (
                Status::Todo,
                "Research async-trait internals",
                "Why is it a macro? Why is life a macro? I need to know.",
            ),
            (
                Status::Todo,
                "Go to the gym",
                "Strong body, strong memory management. Don't let your muscles leak like a C++ string.",
            ),
            (
                Status::Completed,
                "Explain ownership to a rubber duck",
                "The duck didn't get it at first, but after a few squeaks, it finally understood move semantics.",
            ),
            (
                Status::Todo,
                "Check for compiler updates",
                "Is there a new nightly? I need those experimental features for no reason at all.",
            ),
            (
                Status::Todo,
                "Debug the deadlocked thread",
                "It's not a bug, it's just a very dedicated pause in execution.",
            ),
            (
                Status::Completed,
                "Star every crate I use on GitHub",
                "Showing love to the maintainers who keep my 'cargo build' alive.",
            ),
            (
                Status::Todo,
                "Write a custom proc-macro",
                "I want to generate code that generates code. We need to go deeper.",
            ),
            (
                Status::Todo,
                "Finally learn how Pin works",
                "I've read the docs five times. Maybe the sixth time is the charm?",
            ),
            (
                Status::Completed,
                "Delete the 'node_modules' folder",
                "Regained 40GB of disk space. Nature is healing.",
            ),
            (
                Status::Todo,
                "Optimize the Docker image size",
                "From 1.2GB to 10MB using multi-stage builds and Alpine. Efficiency feels good.",
            ),
            (
                Status::Todo,
                "Fix the CI/CD pipeline",
                "It works on my machine, but GitHub Actions thinks otherwise.",
            ),
            (
                Status::Completed,
                "Replace a 5-line bash script with 200 lines of Rust",
                "Was it worth it? Yes. It's type-safe now.",
            ),
            (
                Status::Todo,
                "Actually write documentation",
                "The code is the documentation... said no one ever (and lived).",
            ),
            (
                Status::Todo,
                "Try a different terminal emulator",
                "Is Alacritty faster than Kitty? Let the benchmarking begin.",
            ),
            (
                Status::Completed,
                "Nap for 20 minutes",
                "Brain was stuck in a recursion loop. Had to reboot the system.",
            ),
            (
                Status::Todo,
                "Configure a status bar for the TWM",
                "I need to see my CPU temperature in 16-bit color at all times.",
            ),
            (
                Status::Todo,
                "Rewrite the parser using Nom",
                "Regex is great, but combinators are cooler.",
            ),
            (
                Status::Completed,
                "Find a cool Ferris sticker",
                "My laptop lid still has 2 square inches of empty space.",
            ),
            (
                Status::Todo,
                "Master Vim motions",
                "H, J, K, L... why did I just delete the entire main function?",
            ),
            (
                Status::Todo,
                "Experiment with WebAssembly",
                "Rust in the browser is the future, and the future is now.",
            ),
            (
                Status::Completed,
                "Add a dark mode to the UI",
                "My eyes are no longer burning. Success.",
            ),
            (
                Status::Todo,
                "Submit a PR to an open-source project",
                "Fixing a typo counts as a contribution, right?",
            ),
            (
                Status::Todo,
                "Explain Cow to a non-programmer",
                "No, not the animal. It's 'Clone On Write'. Why are you walking away?",
            ),
            (
                Status::Completed,
                "Organize the physical desk",
                "Found a missing USB drive and three half-empty coffee mugs.",
            ),
            (
                Status::Todo,
                "Write unit tests for the edge cases",
                "What happens if the input is an emoji and a null byte? Let's find out.",
            ),
            (
                Status::Todo,
                "Learn a new functional programming language",
                "Just to see how it feels. Don't worry Rust, I'm not leaving you.",
            ),
            (
                Status::Completed,
                "Solve a LeetCode hard in Rust",
                "The borrow checker was the real final boss.",
            ),
            (
                Status::Todo,
                "Set up a Raspberry Pi server",
                "It will sit in the corner and run a Telegram bot for two days before I forget about it.",
            ),
            (
                Status::Todo,
                "Refactor the error handling",
                "Switching from 'unwrap()' to 'thiserror' because I'm a professional now.",
            ),
            (
                Status::Completed,
                "Explain the orphan rule to a coworker",
                "I think I confused them more, but I feel smarter.",
            ),
            (
                Status::Todo,
                "Benchmark the hot path",
                "Flamegraphs are like modern art, but for nerds.",
            ),
            (
                Status::Todo,
                "Buy a more ergonomic chair",
                "My back is screaming in a language I don't understand.",
            ),
            (
                Status::Completed,
                "Finally finish this list",
                "36 items in total. Now I just have to actually do them.",
            ),
        ])
    }
}

impl FromIterator<(Status, &'static str, &'static str)> for Model {
    fn from_iter<I: IntoIterator<Item = (Status, &'static str, &'static str)>>(iter: I) -> Self {
        let items: Vec<_> = iter
            .into_iter()
            .map(|(status, todo, info)| TodoItem::new(status, todo, info))
            .collect();

        let state = ListState::default().with_selected(Some(0));

        Self { items, state }
    }
}

impl TodoItem {
    fn new(status: Status, todo: &str, info: &str) -> Self {
        Self {
            status,
            todo: todo.to_string(),
            info: info.to_string(),
        }
    }
}

impl Model {
    pub fn selected(&self) -> Option<&TodoItem> {
        self.state
            .selected()
            .and_then(|index| self.items.get(index))
    }

    pub fn select_none(self) -> Self {
        let state = ListState::default().with_selected(None);

        Self {
            items: self.items,
            state,
        }
    }

    pub fn select_first(self) -> Self {
        let state = ListState::default().with_selected(Some(0));

        Model {
            items: self.items,
            state,
        }
    }

    pub fn select_last(self) -> Self {
        let state = ListState::default().with_selected(Some(self.items.len() - 1));

        Model {
            items: self.items,
            state,
        }
    }

    pub fn select_next(self) -> Self {
        match self.state.selected() {
            Some(index) => {
                let state =
                    ListState::default().with_selected(Some((index + 1) % self.items.len()));

                Model { state, ..self }
            }
            None => self.select_first(),
        }
    }

    pub fn select_previous(self) -> Self {
        match self.state.selected() {
            Some(0) => self.select_last(),
            Some(index) => {
                let state = ListState::default().with_selected(Some(index.saturating_sub(1)));
                Model { state, ..self }
            }
            None => self.select_last(),
        }
    }

    pub fn toggle_status(mut self) -> Self {
        let selected_index = self.state.selected();

        match selected_index {
            Some(index) => {
                self.items[index].status = match self.items[index].status {
                    Status::Todo => Status::Completed,
                    Status::Completed => Status::Todo,
                };

                Model { ..self }
            }
            None => Model { ..self },
        }
    }
}
