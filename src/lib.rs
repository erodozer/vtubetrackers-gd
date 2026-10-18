use godot::prelude::*;

struct VtubeTrackersExtension;

#[gdextension]
unsafe impl ExtensionLibrary for VtubeTrackersExtension {

}

use burn::prelude::*;
use vtube_trackers::{FaceBlendshapes, FaceDetector, FaceLandmarker, solve_face};
use image::RgbImage;


// it's recommended to wrap this object with Godot's multi-threaded worker API
// to avoid blocking the main loop when performing inference
#[derive(GodotClass)]
#[class(base=RefCounted)]
pub struct VtFaceDetection {

    face_detector: FaceDetector,
    face_landmarker: FaceLandmarker,
    face_blendshapes: FaceBlendshapes,

    base: Base<RefCounted>,
}

#[godot_api]
impl IRefCounted for VtFaceDetection {
    fn init(base: Base<RefCounted>) -> Self {
        // select the best available GPU device on your system
        let device = Device::wgpu(DeviceKind::DefaultDevice);

        let detector_model = godot::classes::ProjectSettings::singleton().globalize_path(
            "res://addons/vtube_trackers/models/face_detector.bpk"
        ).to_string();

        let landmarker_model = godot::classes::ProjectSettings::singleton().globalize_path(
            "res://addons/vtube_trackers/models/face_landmarks_detector.bpk"
        ).to_string();

        let blendshapes_model = godot::classes::ProjectSettings::singleton().globalize_path(
            "res://addons/vtube_trackers/models/face_blendshapes.bpk"
        ).to_string();


        Self {
            base,
            face_detector: FaceDetector::new(detector_model, &device),
            face_landmarker: FaceLandmarker::new(landmarker_model, &device),
            face_blendshapes: FaceBlendshapes::new(blendshapes_model, &device)
        }
    }
}

#[godot_api]
impl VtFaceDetection {
    #[func]
    fn submit(&self, img: Gd<godot::classes::Image>) -> VarDictionary {
        let data = img.get_data();
        let face = RgbImage::from_raw(
            img.get_width() as u32,
            img.get_height() as u32,
            data.to_vec()
        ).unwrap();

        // Detect faces in image
        let detections = self.face_detector.run_detection(&face, None, None).unwrap();
        let landmarks = self.face_landmarker.run_detection(&face, &detections).unwrap();
        for detection in &landmarks {
            let blendshapes = self.face_blendshapes.run_detection(&detection.screen_landmarks).unwrap();
            let data = PackedFloat32Array::from_iter(blendshapes.into_iter());

            let m44 = solve_face(&face, &detection);
            let transform = Transform3D::new(
                Basis::from_rows(
                    Vector3::new(m44[(0,0)], m44[(0,1)], m44[(0,2)]),
                    Vector3::new(m44[(1,0)], m44[(1,1)], m44[(1,2)]),
                    Vector3::new(m44[(2,0)], m44[(2,1)], m44[(2,2)]),
                ),
                Vector3::new(m44[(0,3)], m44[(1,3)], m44[(2,3)]),
            );

            let mut result = VarDictionary::new();
            result.set("blendshapes", &data);
            result.set("transform", transform);

            return result;
        }

        return VarDictionary::default();
    }
}
