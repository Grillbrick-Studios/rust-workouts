pub mod action;
pub mod item;

use item::MenuItem;

#[derive(Clone)]
pub struct Menu {
  pub heading: String,
  pub items: Vec<MenuItem>,
}

impl Menu {
  fn menu(&self) -> terminal_menu::TerminalMenu {
    let heading = terminal_menu::label(&self.heading);
    let mut heading = vec![heading];
    let mut items: Vec<terminal_menu::TerminalMenuItem> =
      self.items.iter().map(|i| terminal_menu::button(&i.title)).collect();
    heading.append(&mut items);
    terminal_menu::menu(heading)
  }

  pub fn run(&self) -> &MenuItem {
    let menu = self.menu();
    terminal_menu::run(&menu);
    let index = terminal_menu::mut_menu(&menu).selected_item_index();
    let item = &self.items[index];
    (item.action.run)();
    item
  }
}
