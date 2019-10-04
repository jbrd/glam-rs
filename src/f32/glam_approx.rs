use crate::f32::{Mat2, Mat3, Mat4, Quat, Vec2, Vec3, Vec4};
use approx::{AbsDiffEq, RelativeEq, UlpsEq};

impl AbsDiffEq for Quat {
    type Epsilon = <f32 as AbsDiffEq>::Epsilon;
    fn default_epsilon() -> Self::Epsilon {
        f32::default_epsilon()
    }
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        let q1 = self.as_ref();
        let q2 = other.as_ref();
        q1.abs_diff_eq(q2, epsilon)
    }
}

impl RelativeEq for Quat {
    fn default_max_relative() -> Self::Epsilon {
        f32::default_max_relative()
    }
    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        let q1 = self.as_ref();
        let q2 = other.as_ref();
        q1.relative_eq(q2, epsilon, max_relative)
    }
}

impl UlpsEq for Quat {
    fn default_max_ulps() -> u32 {
        f32::default_max_ulps()
    }
    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        let q1 = self.as_ref();
        let q2 = other.as_ref();
        q1.ulps_eq(q2, epsilon, max_ulps)
    }
}

impl AbsDiffEq for Vec2 {
    type Epsilon = <f32 as AbsDiffEq>::Epsilon;
    fn default_epsilon() -> Self::Epsilon {
        f32::default_epsilon()
    }
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        let v1 = self.as_ref();
        let v2 = other.as_ref();
        v1.abs_diff_eq(v2, epsilon)
    }
}

impl RelativeEq for Vec2 {
    fn default_max_relative() -> Self::Epsilon {
        f32::default_max_relative()
    }
    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        let v1 = self.as_ref();
        let v2 = other.as_ref();
        v1.relative_eq(v2, epsilon, max_relative)
    }
}

impl UlpsEq for Vec2 {
    fn default_max_ulps() -> u32 {
        f32::default_max_ulps()
    }
    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        let v1 = self.as_ref();
        let v2 = other.as_ref();
        v1.ulps_eq(v2, epsilon, max_ulps)
    }
}

impl AbsDiffEq for Vec3 {
    type Epsilon = <f32 as AbsDiffEq>::Epsilon;
    fn default_epsilon() -> Self::Epsilon {
        f32::default_epsilon()
    }
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        let v1 = self.as_ref();
        let v2 = other.as_ref();
        v1.abs_diff_eq(v2, epsilon)
    }
}

impl RelativeEq for Vec3 {
    fn default_max_relative() -> Self::Epsilon {
        f32::default_max_relative()
    }
    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        let v1 = self.as_ref();
        let v2 = other.as_ref();
        v1.relative_eq(v2, epsilon, max_relative)
    }
}

impl UlpsEq for Vec3 {
    fn default_max_ulps() -> u32 {
        f32::default_max_ulps()
    }
    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        let v1 = self.as_ref();
        let v2 = other.as_ref();
        v1.ulps_eq(v2, epsilon, max_ulps)
    }
}

impl AbsDiffEq for Vec4 {
    type Epsilon = <f32 as AbsDiffEq>::Epsilon;
    fn default_epsilon() -> Self::Epsilon {
        f32::default_epsilon()
    }
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        let v1 = self.as_ref();
        let v2 = other.as_ref();
        v1.abs_diff_eq(v2, epsilon)
    }
}

impl RelativeEq for Vec4 {
    fn default_max_relative() -> Self::Epsilon {
        f32::default_max_relative()
    }
    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        let v1 = self.as_ref();
        let v2 = other.as_ref();
        v1.relative_eq(v2, epsilon, max_relative)
    }
}

impl UlpsEq for Vec4 {
    fn default_max_ulps() -> u32 {
        f32::default_max_ulps()
    }
    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        let v1 = self.as_ref();
        let v2 = other.as_ref();
        v1.ulps_eq(v2, epsilon, max_ulps)
    }
}

impl AbsDiffEq for Mat2 {
    type Epsilon = <f32 as AbsDiffEq>::Epsilon;
    fn default_epsilon() -> Self::Epsilon {
        f32::default_epsilon()
    }
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        let m1 = self.as_ref();
        let m2 = other.as_ref();
        m1.abs_diff_eq(m2, epsilon)
    }
}

impl RelativeEq for Mat2 {
    fn default_max_relative() -> Self::Epsilon {
        f32::default_max_relative()
    }
    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        let m1 = self.as_ref();
        let m2 = other.as_ref();
        m1.relative_eq(m2, epsilon, max_relative)
    }
}

impl UlpsEq for Mat2 {
    fn default_max_ulps() -> u32 {
        f32::default_max_ulps()
    }
    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        let m1 = self.as_ref();
        let m2 = other.as_ref();
        m1.ulps_eq(m2, epsilon, max_ulps)
    }
}

impl AbsDiffEq for Mat3 {
    type Epsilon = <f32 as AbsDiffEq>::Epsilon;
    fn default_epsilon() -> Self::Epsilon {
        f32::default_epsilon()
    }
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.x_axis().abs_diff_eq(&other.x_axis(), epsilon)
            && self.y_axis().abs_diff_eq(&other.y_axis(), epsilon)
            && self.z_axis().abs_diff_eq(&other.z_axis(), epsilon)
    }
}

impl RelativeEq for Mat3 {
    fn default_max_relative() -> Self::Epsilon {
        f32::default_max_relative()
    }
    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        self.x_axis()
            .relative_eq(&other.x_axis(), epsilon, max_relative)
            && self
                .y_axis()
                .relative_eq(&other.y_axis(), epsilon, max_relative)
            && self
                .z_axis()
                .relative_eq(&other.z_axis(), epsilon, max_relative)
    }
}

impl UlpsEq for Mat3 {
    fn default_max_ulps() -> u32 {
        f32::default_max_ulps()
    }
    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        self.x_axis().ulps_eq(&other.x_axis(), epsilon, max_ulps)
            && self.y_axis().ulps_eq(&other.y_axis(), epsilon, max_ulps)
            && self.z_axis().ulps_eq(&other.z_axis(), epsilon, max_ulps)
    }
}

impl AbsDiffEq for Mat4 {
    type Epsilon = <f32 as AbsDiffEq>::Epsilon;
    fn default_epsilon() -> Self::Epsilon {
        f32::default_epsilon()
    }
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        let m1 = self.as_ref();
        let m2 = other.as_ref();
        m1.abs_diff_eq(m2, epsilon)
    }
}

impl RelativeEq for Mat4 {
    fn default_max_relative() -> Self::Epsilon {
        f32::default_max_relative()
    }
    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        let m1 = self.as_ref();
        let m2 = other.as_ref();
        m1.relative_eq(m2, epsilon, max_relative)
    }
}

impl UlpsEq for Mat4 {
    fn default_max_ulps() -> u32 {
        f32::default_max_ulps()
    }
    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        let m1 = self.as_ref();
        let m2 = other.as_ref();
        m1.ulps_eq(m2, epsilon, max_ulps)
    }
}
