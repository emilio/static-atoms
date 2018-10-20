#[repr(C)]
struct nsStaticAtom {
    mSecret: u32,
}

struct nsGkAtoms {
    mAtoms: [nsStaticAtom; 100],
}

extern {
    static gAtoms: nsGkAtoms;
}

struct Atom(*const nsStaticAtom);
unsafe impl Send for Atom {}
unsafe impl Sync for Atom {}

static MY_ATOM: Atom = Atom(unsafe { &gAtoms.mAtoms[0] });

fn main() {
    println!("{:x}", unsafe { (*MY_ATOM.0).mSecret });
}
