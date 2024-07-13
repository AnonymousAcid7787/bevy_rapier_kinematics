use bevy::prelude::{App, Bundle, Component, Entity, Last, Plugin};
use bevy::utils::default;
use k::{Chain, connect, Error, InverseKinematicsSolver, Isometry3, JacobianIkSolver, JointType, NodeBuilder, RealField, SerialChain, SubsetOf};
use crate::arm::{AttachedArmChain, EntityChain};
use crate::ik_systems::prepare_ik_nodes;


pub struct IKPlugin;

impl Plugin for IKPlugin {
    fn build(&self, app: &mut App) {

        app
            .add_systems(Last, prepare_ik_nodes);
    }
}


#[derive(Component)]
pub struct JacobianIKArm<T: RealField> {
    pub chain: SerialChain<T>,
    pub ik_solver: JacobianIkSolver<T>,
    pub target_pos: Option<k::Translation3<T>>,
    pub elbow_ik_pole: Option<Entity>,
}

impl<T> JacobianIKArm<T>
where
    T: RealField + SubsetOf<f64>
{
    pub fn solve(&mut self, target: &Isometry3<T>) -> Result<(), Error> {
        self.ik_solver.solve(&self.chain, target)
    }
}

impl<T> Default for JacobianIKArm<T>
where 
    T: RealField + SubsetOf<f64>
{
    fn default() -> Self {
        let fixed: k::Node<T> = NodeBuilder::new()
            .name("fixed")
            .joint_type(JointType::Fixed)
            .finalize()
            .into();
        let l0: k::Node<T> = NodeBuilder::new()
            .name("shoulder_y")
            .joint_type(JointType::Rotational {
                axis: k::Vector3::y_axis(),
            })
            .finalize()
            .into();
        let l1: k::Node<T> = NodeBuilder::new()
            .name("shoulder_x")
            .joint_type(JointType::Rotational {
                axis: k::Vector3::x_axis(),
            })
            .finalize()
            .into();
        let l2: k::Node<T> = NodeBuilder::new()
            .name("shoulder_z")
            .joint_type(JointType::Rotational {
                axis: k::Vector3::z_axis(),
            })
            .finalize()
            .into();
        let l3: k::Node<T> = NodeBuilder::new()
            .name("elbow_y")
            .joint_type(JointType::Rotational {
                axis: k::Vector3::y_axis(),
            })
            .finalize()
            .into();
        let l4: k::Node<T> = NodeBuilder::new()
            .name("wrist_z")
            .joint_type(JointType::Rotational {
                axis: k::Vector3::z_axis(),
            })
            .finalize()
            .into();
        let l5: k::Node<T> = NodeBuilder::new()
            .name("wrist_y")
            .joint_type(JointType::Rotational {
                axis: k::Vector3::y_axis(),
            })
            .finalize()
            .into();
        let l6: k::Node<T> = NodeBuilder::new()
            .name("wrist_x")
            .joint_type(JointType::Rotational {
                axis: k::Vector3::x_axis(),
            })
            .finalize()
            .into();
        connect![fixed => l0 => l1 => l2 => l3 => l4 => l5 => l6];
        
        let chain = SerialChain::new_unchecked(Chain::from_root(fixed));
        chain.update_transforms();

        Self {
            chain,
            ik_solver: JacobianIkSolver::default(),
            elbow_ik_pole: None,
            target_pos: None,
        }
    }
}

#[derive(Bundle)]
pub struct JacobianIKArmBundle<T: RealField> {
    pub entity_chain: EntityChain,
    pub ik_arm: JacobianIKArm<T>,
}

impl<T> JacobianIKArmBundle<T>
where 
    T: RealField + SubsetOf<f64>
{
    pub fn new(arm_chain: AttachedArmChain, elbow_ik_pole: Option<Entity>) -> Self {
        Self {
            entity_chain: arm_chain.into(),
            ik_arm: JacobianIKArm {
                elbow_ik_pole,
                ..default()
            },
        }
    }
}

impl<T> Default for JacobianIKArmBundle<T>
where
    T: RealField + SubsetOf<f64>
{
    fn default() -> Self {
        Self {
            entity_chain: default(),
            ik_arm: default(),
        }
    }
}
