// OpenAOE: An open source reimplementation of Age of Empires (1997)
// Copyright (c) 2016 Kevin Fuller
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use specs;

#[derive(Clone, Debug)]
pub struct TransformComponent {
    // TODO: Move to a vector type
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub rotation: f32,
}

impl specs::Component for TransformComponent {
    type Storage = specs::VecStorage<TransformComponent>;
}

impl TransformComponent {
    pub fn new(x: f32, y: f32, z: f32, rotation: f32) -> TransformComponent {
        TransformComponent {
            x: x,
            y: y,
            z: z,
            rotation: rotation,
        }
    }
}

#[derive(Clone, Debug)]
pub struct VelocityComponent {
    // TODO: Move to a vector type
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl specs::Component for VelocityComponent {
    type Storage = specs::VecStorage<VelocityComponent>;
}

impl VelocityComponent {
    pub fn new() -> VelocityComponent {
        VelocityComponent {
            x: 0f32,
            y: 0f32,
            z: 0f32,
        }
    }
}

#[derive(Clone, Debug)]
pub struct CameraComponent {
}

impl specs::Component for CameraComponent {
    type Storage = specs::HashMapStorage<CameraComponent>;
}

impl CameraComponent {
    pub fn new() -> CameraComponent {
        CameraComponent {}
    }
}

#[derive(Clone, Debug)]
pub struct UnitCommonComponent {
}

impl specs::Component for UnitCommonComponent {
    type Storage = specs::VecStorage<UnitCommonComponent>;
}