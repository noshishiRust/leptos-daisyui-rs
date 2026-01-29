# Quick Start Guide

## 🚀 Get Started in 30 Seconds

```powershell
# 1. Launch the interactive menu
.\launcher.ps1

# 2. Select option [1] to launch the demo app
#    Opens http://127.0.0.1:3000

# 3. Make changes to components in src/components/
#    The demo auto-reloads!
```

## 📋 Most Common Commands

| Task | Command | Description |
|------|---------|-------------|
| 🎨 **Demo** | `.\launcher.ps1 -Task dev` | Launch demo app |
| ✅ **Check** | `.\launcher.ps1 -Task check` | Quick validation |
| 🔧 **Fix** | `.\launcher.ps1 -Task fix` | Fix all issues |
| 🏗️ **Build** | `.\launcher.ps1 -Task build` | Build library |
| 🧪 **Test** | `.\launcher.ps1 -Task test` | Run all tests |
| 📚 **Docs** | `.\launcher.ps1 -Task docs` | View documentation |

## 🎯 Development Workflow

### Option A: Using Launcher (Recommended)
```powershell
# Start developing
.\launcher.ps1 -Task dev

# Before committing
.\launcher.ps1 -Task fix
```

### Option B: Using cargo-make
```bash
# Start developing
cargo make dev

# Before committing
cargo make fix-and-verify
```

### Option C: Using cargo directly
```bash
# Start developing
cd demo
trunk serve

# Before committing
cargo fmt && cargo clippy --fix --allow-dirty && cargo test
```

## 📁 Project Structure

```
leptos-daisyui-rs/
├── src/
│   ├── components/     # 62 daisyUI components (100% coverage)
│   │   ├── button/
│   │   ├── tooltip/    # ← New components
│   │   ├── fab/
│   │   └── ...
│   └── utils/          # Utilities (ClassAttributes, etc.)
├── demo/
│   ├── src/
│   │   ├── demos/      # Component demos
│   │   └── core/       # Layout and navigation
│   ├── input.css       # Tailwind CSS config
│   └── index.html      # Demo entry point
├── Makefile.toml       # cargo-make tasks
├── launcher.ps1        # Interactive launcher
└── Cargo.toml          # Dependencies (including table-rs fork)
```

## 🔧 Adding a New Component

1. **Create component directory**
   ```bash
   mkdir src/components/my_component
   ```

2. **Add required files**
   ```
   my_component/
   ├── mod.rs          # pub use component::*; pub use style::*;
   ├── component.rs    # #[component] fn MyComponent(...)
   └── style.rs        # MyComponentColor, MyComponentSize, etc.
   ```

3. **Export in `src/components/mod.rs`**
   ```rust
   mod my_component;
   pub use my_component::*;
   ```

4. **Create demo in `demo/src/demos/my_component.rs`**
   ```rust
   use crate::core::{ContentLayout, Section};
   use leptos::prelude::*;
   use leptos_daisyui_rs::components::*;

   #[component]
   pub fn MyComponentDemo() -> impl IntoView {
       view! {
           <ContentLayout title="My Component" description="...">
               <Section title="Example">
                   <MyComponent />
               </Section>
           </ContentLayout>
       }
   }
   ```

5. **Add to navigation** in `demo/src/core/layout.rs`
   ```rust
   ComponentItem {
       name: "My Component",
       href: "/components/my_component",
       value: "my_component",
   }
   ```

6. **Add route** in `demo/src/main.rs`
   ```rust
   <Route path=path!("/my_component") view=MyComponentDemo />
   ```

7. **Test it**
   ```powershell
   .\launcher.ps1 -Task dev
   # Visit http://127.0.0.1:3000/components/my_component
   ```

## 🎨 Component Pattern Example

```rust
// component.rs
use crate::merge_classes;
use leptos::{html::Button, prelude::*};

#[component]
pub fn MyComponent(
    #[prop(optional, into)] color: Signal<MyComponentColor>,
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Button>,
    children: Children,
) -> impl IntoView {
    view! {
        <button
            node_ref=node_ref
            class=move || merge_classes!("my-component", color.get().as_str(), class)
        >
            {children()}
        </button>
    }
}

// style.rs
#[derive(Clone, Debug, Default)]
pub enum MyComponentColor {
    #[default]
    Default,
    Primary,
    Secondary,
}

impl MyComponentColor {
    pub fn as_str(&self) -> &'static str {
        match self {
            MyComponentColor::Default => "",
            MyComponentColor::Primary => "my-component-primary",
            MyComponentColor::Secondary => "my-component-secondary",
        }
    }
}
```

## 🐛 Troubleshooting

### Demo won't start
```powershell
# Check if port 3000 is in use
netstat -ano | findstr :3000

# Try alternate port
cd demo
trunk serve --port 8080
```

### Build errors after updating
```powershell
# Clean and rebuild
.\launcher.ps1
# Select option [20] Clean All
# Then option [8] Verify All
```

### Clippy warnings
```powershell
# Auto-fix most issues
.\launcher.ps1 -Task fix
```

### table-rs dependency issues
```powershell
# Ensure table-rs fork exists at ../table-rs
cd ..
git clone <your-fork-url> table-rs
cd leptos-daisyui-rs
cargo build
```

## 📚 Resources

- **Full Launcher Guide**: [`LAUNCHER.md`](./LAUNCHER.md)
- **Project Documentation**: [`CLAUDE.md`](./CLAUDE.md)
- **daisyUI Components**: https://daisyui.com/components/
- **Leptos Documentation**: https://leptos.dev/
- **cargo-make Tasks**: `cargo make --list-all-steps`

## 💡 Tips

1. **Use the launcher menu** for discovery - see all available options
2. **Parallel workflows** - Option [24] runs demo + docs simultaneously
3. **Quick check before commit** - `.\launcher.ps1 -Task check`
4. **Format on save** - Configure your editor to run `cargo fmt` on save
5. **Watch mode** - The demo auto-reloads when you change library code

## 🎯 Next Steps

1. Explore all 62 components in the demo app
2. Modify an existing component and see hot-reload in action
3. Create a custom component following the pattern above
4. Run the full verification suite before committing
5. Build your own Leptos app using leptos-daisyui-rs!

---

**Happy coding! 🦀✨**
