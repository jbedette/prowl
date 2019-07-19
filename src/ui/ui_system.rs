use mod::panel;

impl<'a> System for PanelSystem<'a> {
    type SystemData = (
        ReadStorage<'a, Panel>
        );
}
