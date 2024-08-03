use bevy::prelude::{Added, Component, Entity, Query, RemovedComponents, ResMut};
use bevy_rapier3d::prelude::RapierContext;
use bevy_rapier3d::rapier::prelude::GenericJoint;

#[derive(Component)]
pub struct ToggleContactsWith {
    pub entity: Entity,
    pub contacts_enabled: bool
}

impl ToggleContactsWith {
    #[inline]
    pub fn new(entity: Entity, contacts_enabled: bool) -> Self {
        ToggleContactsWith {
            entity,
            contacts_enabled
        }
    }
}

pub fn toggle_contacts_with(
    mut rapier_context: ResMut<RapierContext>,
    toggled_contacts_q: Query<(Entity, &ToggleContactsWith), Added<ToggleContactsWith>>,
    mut removals: RemovedComponents<ToggleContactsWith>
) {
    for (entity, toggled_contact) in toggled_contacts_q.iter() {
        let this_body = *rapier_context.entity2body().get(&entity).unwrap();
        let other_body = *rapier_context.entity2body().get(&toggled_contact.entity).unwrap();
        rapier_context.impulse_joints.insert(
            this_body,
            other_body,
            *GenericJoint::default().set_contacts_enabled(toggled_contact.contacts_enabled),
            true
        );
    }

    for removed in removals.read() {
        if let Some(joint) = rapier_context.entity2impulse_joint().get(&removed) {
            let handle = *joint;
            rapier_context.impulse_joints.remove(handle, true);
        }
    }
}
