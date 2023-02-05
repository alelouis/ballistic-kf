use linearkalman::{predict_step, update_step, KalmanState};
use rand_distr::{Distribution, Normal};
use rapier2d::prelude::*;
use rulinalg::vector::Vector as ruVec;
// use serde_json::json;
use std::sync::mpsc::Sender;

pub(crate) fn simulation(tx: Sender<[f64; 7]>) {
    let mut rigid_body_set = RigidBodySet::new();
    let mut collider_set = ColliderSet::new();

    /* Create the ground. */
    let collider = ColliderBuilder::cuboid(10000.0, 1.0).build();
    collider_set.insert(collider);

    /* Create the bouncing ball. */
    let rigid_body = RigidBodyBuilder::dynamic()
        .translation(vector![0.0, 1.0])
        .linvel(vector![10.0, 100.0])
        .build();
    let collider = ColliderBuilder::ball(0.5)
        .restitution(1.0)
        .friction(0.0)
        .build();
    let ball_body_handle = rigid_body_set.insert(rigid_body);
    collider_set.insert_with_parent(collider, ball_body_handle, &mut rigid_body_set);

    /* Create other structures necessary for the simulation. */
    let gravity = vector![0.0, -9.81];
    let integration_parameters = IntegrationParameters::default();
    let mut physics_pipeline = PhysicsPipeline::new();
    let mut island_manager = IslandManager::new();
    let mut broad_phase = BroadPhase::new();
    let mut narrow_phase = NarrowPhase::new();
    let mut impulse_joint_set = ImpulseJointSet::new();
    let mut multibody_joint_set = MultibodyJointSet::new();
    let mut ccd_solver = CCDSolver::new();
    let physics_hooks = ();
    let event_handler = ();

    let normal = Normal::new(0.0, 2.0).unwrap();
    let kf = crate::kalman::init_filter();
    let mut last_state = KalmanState {
        x: (kf.x0).clone(),
        p: (kf.p0).clone(),
    };
    let mut time = 0.0;
    let dt = 1. / 60.;

    /* Run the game loop, stepping the simulation once per frame. */
    for _ in 0..3000 {
        time += dt;
        physics_pipeline.step(
            &gravity,
            &integration_parameters,
            &mut island_manager,
            &mut broad_phase,
            &mut narrow_phase,
            &mut rigid_body_set,
            &mut collider_set,
            &mut impulse_joint_set,
            &mut multibody_joint_set,
            &mut ccd_solver,
            None,
            &event_handler,
            &physics_hooks,
        );

        /* Sending simulation data to GUI thread*/
        let ball_body = &rigid_body_set[ball_body_handle];
        let measurements = [
            ball_body.translation().x as f64 + 10.0 * normal.sample(&mut rand::thread_rng()),
            ball_body.translation().y as f64 + 10.0 * normal.sample(&mut rand::thread_rng()),
        ];

        // Kalman predict and update
        let data = ruVec::new([measurements[0], measurements[1]]);
        let kf_pred = predict_step(&kf, &last_state);
        let kf_update = update_step(&kf, &kf_pred, &data);
        last_state = kf_update;

        // Send to GUI
        let payload = [
            time,
            measurements[0],
            measurements[1],
            last_state.x[0],
            last_state.x[1],
            ball_body.translation().x as f64,
            ball_body.translation().y as f64,
        ];

        tx.send(payload)
            .expect("Couldn't send from physics thread to GUI thread.");

        // zmq part
        // let json_payload = json!(payload);
        // let mut msg = zmq::Message::new();
        // requester.send(&json_payload.to_string(), 0).unwrap();
        // requester.recv(&mut msg, 0).unwrap();
    }
}
