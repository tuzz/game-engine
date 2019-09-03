use specs::prelude::*;
use crate::utilities::Vector3f;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Material {
    ambient: Vector3f,
    diffuse: Vector3f,
    specular: Vector3f,
    shininess: f32,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            ambient: Vector3f::new(1.0, 1.0, 1.0),
            diffuse: Vector3f::new(1.0, 1.0, 1.0),
            specular: Vector3f::new(1.0, 1.0, 1.0),
            shininess: 1.0,
        }
    }
}

impl Material {
    fn emerald() -> Self {
        Self {
            ambient: Vector3f::new(0.0215, 0.1745, 0.0215),
            diffuse: Vector3f::new(0.07568, 0.61424, 0.07568),
            specular: Vector3f::new(0.633, 0.727811, 0.633),
            shininess: 0.6,
        }
    }

    fn jade() -> Self {
        Self {
            ambient: Vector3f::new(0.135, 0.2225, 0.1575),
            diffuse: Vector3f::new(0.54, 0.89, 0.63),
            specular: Vector3f::new(0.316228, 0.316228, 0.316228),
            shininess: 0.1,
        }
    }

    fn obsidian() -> Self {
        Self {
            ambient: Vector3f::new(0.05375, 0.05, 0.06625),
            diffuse: Vector3f::new(0.18275, 0.17, 0.22525),
            specular: Vector3f::new(0.332741, 0.328634, 0.346435),
            shininess: 0.3,
        }
    }

    fn pearl() -> Self {
        Self {
            ambient: Vector3f::new(0.25, 0.20725, 0.20725),
            diffuse: Vector3f::new(1.0, 0.829, 0.829),
            specular: Vector3f::new(0.296648, 0.296648, 0.296648),
            shininess: 0.088,
        }
    }

    fn ruby() -> Self {
        Self {
            ambient: Vector3f::new(0.1745, 0.01175, 0.01175),
            diffuse: Vector3f::new(0.61424, 0.04136, 0.04136),
            specular: Vector3f::new(0.727811, 0.626959, 0.626959),
            shininess: 0.6,
        }
    }

    fn turquoise() -> Self {
        Self {
            ambient: Vector3f::new(0.1, 0.18725, 0.1745),
            diffuse: Vector3f::new(0.396, 0.74151, 0.69102),
            specular: Vector3f::new(0.297254, 0.30829, 0.306678),
            shininess: 0.1,
        }
    }

    fn brass() -> Self {
        Self {
            ambient: Vector3f::new(0.329412, 0.223529, 0.027451),
            diffuse: Vector3f::new(0.780392, 0.568627, 0.113725),
            specular: Vector3f::new(0.992157, 0.941176, 0.807843),
            shininess: 0.21794872,
        }
    }

    fn bronze() -> Self {
        Self {
            ambient: Vector3f::new(0.2125, 0.1275, 0.054),
            diffuse: Vector3f::new(0.714, 0.4284, 0.18144),
            specular: Vector3f::new(0.393548, 0.271906, 0.166721),
            shininess: 0.2,
        }
    }

    fn chrome() -> Self {
        Self {
            ambient: Vector3f::new(0.25, 0.25, 0.25),
            diffuse: Vector3f::new(0.4, 0.4, 0.4),
            specular: Vector3f::new(0.774597, 0.774597, 0.774597),
            shininess: 0.6,
        }
    }

    fn copper() -> Self {
        Self {
            ambient: Vector3f::new(0.19125, 0.0735, 0.0225),
            diffuse: Vector3f::new(0.7038, 0.27048, 0.0828),
            specular: Vector3f::new(0.256777, 0.137622, 0.086014),
            shininess: 0.1,
        }
    }

    fn gold() -> Self {
        Self {
            ambient: Vector3f::new(0.24725, 0.1995, 0.0745),
            diffuse: Vector3f::new(0.75164, 0.60648, 0.22648),
            specular: Vector3f::new(0.628281, 0.555802, 0.366065),
            shininess: 0.4,
        }
    }

    fn silver() -> Self {
        Self {
            ambient: Vector3f::new(0.19225, 0.19225, 0.19225),
            diffuse: Vector3f::new(0.50754, 0.50754, 0.50754),
            specular: Vector3f::new(0.508273, 0.508273, 0.508273),
            shininess: 0.4,
        }
    }

    fn black_plastic() -> Self {
        Self {
            ambient: Vector3f::new(0.0, 0.0, 0.0),
            diffuse: Vector3f::new(0.01, 0.01, 0.01),
            specular: Vector3f::new(0.50, 0.50, 0.50),
            shininess: 0.25,
        }
    }

    fn cyan_plastic() -> Self {
        Self {
            ambient: Vector3f::new(0.0, 0.1, 0.06),
            diffuse: Vector3f::new(0.0, 0.50980392, 0.50980392),
            specular: Vector3f::new(0.50196078, 0.50196078, 0.50196078),
            shininess: 0.25,
        }
    }

    fn green_plastic() -> Self {
        Self {
            ambient: Vector3f::new(0.0, 0.0, 0.0),
            diffuse: Vector3f::new(0.1, 0.35, 0.1),
            specular: Vector3f::new(0.45, 0.55, 0.45),
            shininess: 0.25,
        }
    }

    fn red_plastic() -> Self {
        Self {
            ambient: Vector3f::new(0.0, 0.0, 0.0),
            diffuse: Vector3f::new(0.5, 0.0, 0.0),
            specular: Vector3f::new(0.7, 0.6, 0.6),
            shininess: 0.25,
        }
    }

    fn white_plastic() -> Self {
        Self {
            ambient: Vector3f::new(0.0, 0.0, 0.0),
            diffuse: Vector3f::new(0.55, 0.55, 0.55),
            specular: Vector3f::new(0.70, 0.70, 0.70),
            shininess: 0.25,
        }
    }

    fn yellow_plastic() -> Self {
        Self {
            ambient: Vector3f::new(0.0, 0.0, 0.0),
            diffuse: Vector3f::new(0.5, 0.5, 0.0),
            specular: Vector3f::new(0.60, 0.60, 0.50),
            shininess: 0.25,
        }
    }

    fn black_rubber() -> Self {
        Self {
            ambient: Vector3f::new(0.02, 0.02, 0.02),
            diffuse: Vector3f::new(0.01, 0.01, 0.01),
            specular: Vector3f::new(0.4, 0.4, 0.4),
            shininess: 0.078125,
        }
    }

    fn cyan_rubber() -> Self {
        Self {
            ambient: Vector3f::new(0.0, 0.05, 0.05),
            diffuse: Vector3f::new(0.4, 0.5, 0.5),
            specular: Vector3f::new(0.04, 0.7, 0.7),
            shininess: 0.078125,
        }
    }

    fn green_rubber() -> Self {
        Self {
            ambient: Vector3f::new(0.0, 0.05, 0.0),
            diffuse: Vector3f::new(0.4, 0.5, 0.4),
            specular: Vector3f::new(0.04, 0.7, 0.04),
            shininess: 0.078125,
        }
    }

    fn red_rubber() -> Self {
        Self {
            ambient: Vector3f::new(0.05, 0.0, 0.0),
            diffuse: Vector3f::new(0.5, 0.4, 0.4),
            specular: Vector3f::new(0.7, 0.04, 0.04),
            shininess: 0.078125,
        }
    }

    fn white_rubber() -> Self {
        Self {
            ambient: Vector3f::new(0.05, 0.05, 0.05),
            diffuse: Vector3f::new(0.5, 0.5, 0.5),
            specular: Vector3f::new(0.7, 0.7, 0.7),
            shininess: 0.078125,
        }
    }

    fn yellow_rubber() -> Self {
        Self {
            ambient: Vector3f::new(0.05, 0.05, 0.0),
            diffuse: Vector3f::new(0.5, 0.5, 0.4),
            specular: Vector3f::new(0.7, 0.7, 0.04),
            shininess: 0.078125,
        }
    }
}
