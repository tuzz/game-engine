mod generate;

#[derive(Default)]
pub struct Shader {
    pub headers: Vec<String>,
    pub attributes: Vec<Variable>,
    pub uniforms: Vec<Variable>,
    pub varyings: Vec<Variable>,
    pub statements: Vec<String>,
}

pub struct Variable {
    pub kind: String,
    pub name: String,
}

impl Variable {
    pub fn new(kind: &str, name: &str) -> Self {
        Self { kind: kind.to_string(), name: name.to_string() }
    }
}

impl Shader {
    pub fn header(&mut self, header: &str) {
        self.headers.push(header.to_string());
    }

    pub fn attribute(&mut self, kind: &str, name: &str) {
        self.attributes.push(Variable::new(kind, name));
    }

    pub fn uniform(&mut self, kind: &str, name: &str) {
        self.uniforms.push(Variable::new(kind, name));
    }

    pub fn varying(&mut self, kind: &str, name: &str) {
        self.varyings.push(Variable::new(kind, name));
    }

    pub fn statement(&mut self, statement: &str) {
        self.statements.push(statement.to_string());
    }

    pub fn source(&self) -> String {
        self.lines().join("\n")
    }

    pub fn lines(&self) -> Vec<String> {
        let mut lines = vec![];

        for header in &self.headers {
            lines.push(format!("{};", header));
        }

        for var in &self.attributes {
            lines.push(format!("attribute {} {};", var.kind, var.name));
        }

        for var in &self.uniforms {
            lines.push(format!("uniform {} {};", var.kind, var.name));
        }

        for var in &self.varyings {
            lines.push(format!("varying {} {};", var.kind, var.name));
        }

        lines.push("void main() {".to_string());

        for statement in &self.statements {
            lines.push(format!("    {};", statement));
        }

        lines.push("}".to_string());

        lines
    }
}
