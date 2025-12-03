use gpui::{div, App, AppContext, Context, Entity, FocusHandle, IntoElement, Render, Window, ParentElement};
use gpui_component::{IconName, Side};
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Item {
    Library,
    Workflow,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum SubItem {
    Explorer,
    Categories,
    Tags,
}

struct LeftDock {
    active_items: HashMap<Item, bool>,
    last_active_item: Item,
    active_subitems: Option<SubItem>,
    collapsed: bool,
    side: Side,
    focus_handle: FocusHandle,
    checked: bool,
}

impl Item {
    fn label(&self) -> &'static str {
        match self {
            Item::Library => "Library",
            Item::Workflow => "Workflow",
        }
    }

    fn icons(&self) -> IconName {
        match self {
            Item::Library => IconName::Folder,
            Item::Workflow => IconName::SquareTerminal,
        }
    }
}

impl SubItem {
    fn label(&self) -> &'static str {
        match self {
            SubItem::Explorer => "Explorer",
            SubItem::Categories => "Categories",
            SubItem::Tags => "Tags",
        }
    }

    fn icons(&self) -> IconName {
        match self {
            SubItem::Explorer => IconName::Star,
            SubItem::Categories => IconName::Asterisk,
            SubItem::Tags => IconName::Bot,
        }
    }
}

impl LeftDock {
    fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let mut active_items = HashMap::new();
        active_items.insert(Item::Library, true);

        Self {
            active_items,
            last_active_item: Item::Library,
            active_subitems: None,
            collapsed: false,
            side: Side::Left,
            focus_handle: cx.focus_handle(),
            checked: false,
        }
    }

    fn render_content(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}

impl Render for LeftDock {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().child(self.render_content(window, cx))
    }
}
