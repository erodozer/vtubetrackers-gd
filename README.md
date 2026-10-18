# vtubetrackers-gd

Godot bindings for [Vtube-Trackers](https://codeberg.org/MyriadMinds/vtube-trackers), utilizing [Burn](https://github.com/tracel-ai/burn/tree/main) for infering face detection from Mediapipe detection models from plain image data.

## Performance Recommendations

The binding is left intentionally simple and only provides a means of starting up models for Face detection in an explicit manner.
To improve performance and avoid blocking the main loop of a Godot application, you should use multiple threads to split inference scheduling and result read-back.

## Models

Models used in this project are property of Google and licensed under the Apache-2.0 license.  They are the same model files taken from [upstream](https://codeberg.org/MyriadMinds/vtube-trackers/weights).
