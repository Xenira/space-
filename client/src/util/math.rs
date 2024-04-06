use bevy::prelude::*;

pub fn get_transform(
    global_source: &GlobalTransform,
    local_source: &Transform,
    global_target: &GlobalTransform,
) -> Transform {
    let global_source = global_source.compute_transform();
    let global_target = global_target.compute_transform();

    let translation =
        global_source.translation - global_target.translation - local_source.translation;
    let rotation =
        global_source.rotation * local_source.rotation.conjugate() * global_target.rotation;
    let scale = global_source.scale / global_target.scale / local_source.scale;

    Transform {
        translation,
        rotation,
        scale,
    }
}
