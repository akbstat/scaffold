# Scaffold
> A tool for generated sas program template for SDTM, ADaM and TFLs projects

# Usage

```rust
use scaffold::{Generator, Group, Kind, Param};
use Path;

fn main() {
    let dev = Param {
        study: "STUDY".into(),
        engine: "SAS".into(),
        group: Group::Dev,
    };
    let qc = Param {
        study: "STUDY".into(),
        engine: "SAS".into(),
        group: Group::Qc,
    };
    let config = Path::new(
        r"your path\spec.xlsx",
    );
    let template_dir = Path::new(r"your tempalte folder");
    let dev_dest = Path::new(r"your destination folder");
    let qc_dest = Path::new(r"your destination folder");
    let g = Generator::new(config, template_dir, Kind::SDTM).unwrap();
    g.render(dev_dest, &dev).unwrap();
    g.render(qc_dest, &qc).unwrap();
}
```