use serde_json::Value;

pub struct Activity {

}

impl Activity {
    pub fn new(data: &Value) -> Self {
        Activity { 

        }
    }

}

// -- TODO --
// durch weatch hisotry loopen
// dann die zeit rausfiltern und convertern (parser in helper func)
// anschließend durch alle zeiten loopen und miteinander vergliechen
// wenn abstand zwischen angeschauten videos >5 min oder so endet eine watch session (eignes struct für jede watchsession mit date watched und watch time (alle abstände der zeiten addiert in der session)) und allees gepseichert im einen array