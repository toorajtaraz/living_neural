const ACTIVATION_WORMS: &'static str = r#"

float inverse_gaussian(float x) {
  return -1./pow(2., (0.6*pow(x, 2.)))+1.;
}

float activation(float x) {
  return inverse_gaussian(x);
}	

"#;
const ACTIVATION_WAVES: &'static str = r#"

float activation(float x) {
  return abs(1.2*x);
}

"#;
const ACTIVATION_GAME_OF_LIFE: &'static str = r#"

    float activation(float x) {
      if (x == 3. || x == 11. || x == 12.){
        return 1.;
      }
      return 0.;
    }	

"#;
const ACTIVATION_RULE30: &'static str = r#"

    float activation(float x) {
      if (x == 1. || x == 2. || x == 3.|| x == 4.){
        return 1.;
      }
      return 0.;
    }		

"#;
const ACTIVATION_POWER: &'static str = r#"

    float activation(float x) {
      return pow(x, 2.);
    }

"#;
const ACTIVATION_SIN: &'static str = r#"

    float activation(float x) {
      return sin(x);
    }

"#;
const ACTIVATION_ABSOLUTE: &'static str = r#"

    float activation(float x) {
      return abs(x);
    }

"#;
const ACTIVATION_TANH: &'static str = r#"

    float activation(float x) {
      return (exp(2.*x)-1.)/(exp(2.*x)+1.);
    }

"#;
const ACTIVATION_INVERSE_GAUSSIAN: &'static str = r#"

    float activation(float x) {
      return -1./pow(2., (pow(x, 2.)))+1.;
    }

"#;
const ACTIVATION_IDENTITY: &'static str = r#"

    float activation(float x) {
      return x;
    }

"#;
const PERSISTENT_SRC: &'static str = r#"

    float cur = texture(u_plane, get_point(v_text_points, vec2(0.0, 0.0))).a;
    if (cur != 0.) {
        color = vec4(cur, cur, cur, cur);
        return;
    }

"#;
const FRAGMENT_SRC: &'static str = r#" 
    #version 450
    precision mediump float;
    in vec2 v_text_points;
    out vec4 color;

    uniform vec2 u_single_pixel;
    uniform vec4 u_color_mask;
    uniform mat3 u_kernel;
    uniform sampler2D u_plane;
    uniform bool u_do_calc;
    uniform sampler2D u_plane_out;

    vec2 get_point(vec2 point, vec2 offset) {
        return fract(point + u_single_pixel * offset);
    }

    ACTIVATION_SRC

    void main() {
        if (u_do_calc) {

            PERSISTENT_SRC

            float conv_res_a =
                      texture(u_plane, get_point(v_text_points, vec2( 1.,-1.))).a * u_kernel[0][0]
                    + texture(u_plane, get_point(v_text_points, vec2( 0.,-1.))).a * u_kernel[1][0]
                    + texture(u_plane, get_point(v_text_points, vec2(-1.,-1.))).a * u_kernel[2][0]
                    + texture(u_plane, get_point(v_text_points, vec2( 1., 0.))).a * u_kernel[0][1]
                    + texture(u_plane, get_point(v_text_points, vec2( 0., 0.))).a * u_kernel[1][1]
                    + texture(u_plane, get_point(v_text_points, vec2(-1., 0.))).a * u_kernel[2][1]
                    + texture(u_plane, get_point(v_text_points, vec2( 1., 1.))).a * u_kernel[0][2]
                    + texture(u_plane, get_point(v_text_points, vec2( 0., 1.))).a * u_kernel[1][2]
                    + texture(u_plane, get_point(v_text_points, vec2(-1., 1.))).a * u_kernel[2][2];
            float activated = activation(conv_res_a);
            color = vec4(activated, activated, activated, activated);
        } else {
            float x = texture(u_plane, v_text_points).a;
			color = vec4(x, x, x, x) * u_color_mask;
        }
    }
"#;

#[allow(dead_code)]
pub enum Activation {
    SIN,
    TANH,
    POWER,
    RULE30,
    ABSOLUTE,
    IDENTITY,
    GAMEOFLIFE,
    INVERSEGAUSSIAN,
    WAVES,
    WORMS,
    CUSTOM,
}

pub fn get_fragment_shader(
    activation: Activation,
    is_persistent: bool,
    custom_activation: Option<String>,
) -> String {
    let mut fragment_src = String::from(FRAGMENT_SRC);
    if is_persistent {
        fragment_src = fragment_src.replace("PERSISTENT_SRC", PERSISTENT_SRC);
    } else {
        fragment_src = fragment_src.replace("PERSISTENT_SRC", " ");
    }
    match activation {
        Activation::SIN => {
            fragment_src = fragment_src.replace("ACTIVATION_SRC", ACTIVATION_SIN);
        }
        Activation::TANH => {
            fragment_src = fragment_src.replace("ACTIVATION_SRC", ACTIVATION_TANH);
        }
        Activation::POWER => {
            fragment_src = fragment_src.replace("ACTIVATION_SRC", ACTIVATION_POWER);
        }
        Activation::RULE30 => {
            fragment_src = fragment_src.replace("ACTIVATION_SRC", ACTIVATION_RULE30);
        }
        Activation::ABSOLUTE => {
            fragment_src = fragment_src.replace("ACTIVATION_SRC", ACTIVATION_ABSOLUTE);
        }
        Activation::IDENTITY => {
            fragment_src = fragment_src.replace("ACTIVATION_SRC", ACTIVATION_IDENTITY);
        }
        Activation::GAMEOFLIFE => {
            fragment_src = fragment_src.replace("ACTIVATION_SRC", ACTIVATION_GAME_OF_LIFE);
        }
        Activation::INVERSEGAUSSIAN => {
            fragment_src = fragment_src.replace("ACTIVATION_SRC", ACTIVATION_INVERSE_GAUSSIAN);
        }
        Activation::WAVES => {
            fragment_src = fragment_src.replace("ACTIVATION_SRC", ACTIVATION_WAVES);
        }
        Activation::WORMS => {
            fragment_src = fragment_src.replace("ACTIVATION_SRC", ACTIVATION_WORMS);
        }
        Activation::CUSTOM => {
            fragment_src = fragment_src.replace(
                "ACTIVATION_SRC",
                custom_activation
                    .unwrap_or(ACTIVATION_IDENTITY.to_owned())
                    .as_str(),
            );
        }
    }
    fragment_src
}
