# Table Component Comparison: leptos-daisyui-rs vs opensass/table-rs

## 📊 Executive Summary

**Recommendation:** ✅ **YES - Adapt table-rs for leptos-daisyui-rs**

The opensass/table-rs library provides advanced table functionality that would significantly enhance leptos-daisyui-rs. While they haven't implemented Leptos support yet, we can be the first to create a Leptos implementation using their proven architecture.

---

## 🔍 Current State Comparison

### leptos-daisyui-rs Table (Current)

**Type:** Basic styled wrappers
**Lines of Code:** ~220 lines
**Complexity:** Low

**Components:**
- `Table` - Main table wrapper
- `TableHead` - `<thead>` wrapper
- `TableBody` - `<tbody>` wrapper
- `TableFoot` - `<tfoot>` wrapper
- `TableRow` - `<tr>` wrapper
- `TableHeader` - `<th>` wrapper
- `TableCell` - `<td>` wrapper

**Features:**
- ✅ Size variants (xs, sm, md, lg, xl)
- ✅ Zebra striping
- ✅ Pin rows (sticky header/footer)
- ✅ Pin columns (sticky first column)
- ✅ daisyUI theme integration
- ❌ No data management
- ❌ No sorting
- ❌ No pagination
- ❌ No search/filtering
- ❌ No loading states

**Pros:**
- Simple, lightweight
- daisyUI styled
- Full control over markup
- Easy to understand

**Cons:**
- Requires manual implementation of all logic
- No built-in interactivity
- User must handle data, sorting, pagination, search themselves

---

### opensass/table-rs (Reference)

**Type:** Full-featured data table
**Lines of Code:** ~500+ lines (Yew implementation)
**Complexity:** Medium-High

**Components:**
- `Table` - Main smart table component
- `TableHeader` - Header with sorting controls
- `TableBody` - Body with loading/empty states
- `PaginationControls` - Pagination UI

**Features:**
- ✅ Column-based data structure
- ✅ Built-in sorting (ascending/descending)
- ✅ Pagination with page controls
- ✅ Search/filtering with debounce
- ✅ URL query sync (`?search=term&page=2`)
- ✅ Loading states
- ✅ Empty states
- ✅ Custom renderers per column
- ✅ Sortable column configuration
- ✅ Min-width column sizing
- ✅ ARIA accessibility (aria-sort, roles)
- ✅ Fully customizable classes
- ✅ Fully customizable text labels
- ✅ Inline style support

**Pros:**
- Complete data table solution
- Production-ready features
- Excellent performance (WASM optimized)
- Proven in Yew/Dioxus
- Well documented
- Customizable styling

**Cons:**
- No Leptos support yet (marked "TODO")
- More complex API
- Heavier bundle size

---

## 🎯 Feature Comparison Matrix

| Feature | daisyUI Table | table-rs | Priority |
|---------|---------------|----------|----------|
| **Basic Styling** | ✅ | ✅ | ✓ |
| **Size Variants** | ✅ (5 sizes) | ⚠️ (custom CSS) | Medium |
| **Zebra Striping** | ✅ | ⚠️ (custom CSS) | Low |
| **Sticky Header/Footer** | ✅ | ⚠️ (custom CSS) | Low |
| **Sticky Columns** | ✅ | ⚠️ (custom CSS) | Low |
| **Data Management** | ❌ | ✅ | **HIGH** |
| **Column Definitions** | ❌ | ✅ | **HIGH** |
| **Sorting** | ❌ | ✅ | **HIGH** |
| **Pagination** | ❌ | ✅ | **HIGH** |
| **Search/Filter** | ❌ | ✅ | **HIGH** |
| **Loading States** | ❌ | ✅ | **HIGH** |
| **Empty States** | ❌ | ✅ | Medium |
| **URL Sync** | ❌ | ✅ | Medium |
| **Custom Renderers** | ✅ (via children) | ✅ (via accessor) | Medium |
| **Accessibility** | ⚠️ (basic) | ✅ (aria-sort, roles) | **HIGH** |
| **i18n Text Support** | ❌ | ✅ | Medium |
| **Theme Integration** | ✅ daisyUI | ❌ | **HIGH** |

**Legend:**
- ✅ = Fully supported
- ⚠️ = Partially supported or requires custom work
- ❌ = Not supported

---

## 📈 Performance Benchmarks

According to table-rs documentation:

### Table RS (Yew + WASM) vs TanStack Table (React)

| Metric | TanStack Table | table-rs | Winner |
|--------|----------------|----------|--------|
| **Page Load (1M rows)** | ~10s | ~2s | 🏆 table-rs (5x faster) |
| **Memory Usage** | >3GB (heap overflow) | ~1.1GB | 🏆 table-rs (3x less) |
| **Sorting Performance** | 2-4s | <1s | 🏆 table-rs (4x faster) |
| **Search Performance** | Slow | Instantaneous | 🏆 table-rs |
| **Lighthouse Score** | 49/100 | 60/100 | 🏆 table-rs |

**Key Takeaway:** WASM-compiled table logic is **significantly more efficient** than JavaScript for large datasets.

---

## 🏗️ Architecture Comparison

### Current daisyUI Table Architecture

```rust
// Simple wrapper pattern
#[component]
pub fn Table(
    size: Signal<TableSize>,
    zebra: Signal<bool>,
    pin_rows: Signal<bool>,
    pin_cols: Signal<bool>,
    children: Children,
) -> impl IntoView {
    view! {
        <table class="table ...">
            {children()}
        </table>
    }
}

// Manual usage - user handles everything
view! {
    <Table zebra=true>
        <TableHead>
            <TableRow>
                <TableHeader>"Name"</TableHeader>
                <TableHeader>"Email"</TableHeader>
            </TableRow>
        </TableHead>
        <TableBody>
            <For
                each=move || data.get()
                key=|item| item.id
                children=|item| view! {
                    <TableRow>
                        <TableCell>{item.name}</TableCell>
                        <TableCell>{item.email}</TableCell>
                    </TableRow>
                }
            />
        </TableBody>
    </Table>
}
```

**Pros:**
- Full control
- Simple API
- Flexible

**Cons:**
- User must implement sorting, pagination, search, loading states
- Lots of boilerplate
- No built-in data management

---

### Proposed table-rs-inspired Architecture

```rust
// Smart component pattern
#[component]
pub fn DataTable(
    data: Signal<Vec<HashMap<&'static str, String>>>,
    columns: Vec<Column>,
    page_size: Option<usize>,
    sortable: bool,
    searchable: bool,
    paginate: bool,
    // ... daisyUI-specific props
    size: Signal<TableSize>,
    zebra: Signal<bool>,
    pin_rows: Signal<bool>,
) -> impl IntoView {
    // Internal state management
    let page = create_rw_signal(0);
    let sort_column = create_rw_signal(None);
    let search_query = create_rw_signal(String::new());

    // Computed filtered/sorted/paginated data
    let displayed_data = create_memo(move |_| {
        let mut data = data.get();
        // Apply search filter
        // Apply sort
        // Apply pagination
        data
    });

    view! {
        <div class="table-container">
            {move || if searchable {
                view! { <input type="search" /> }
            } else {
                view! {}
            }}

            <table class="table">
                <thead>
                    <tr>
                        <For
                            each=move || columns.clone()
                            key=|col| col.id
                            children=|col| view! {
                                <th on:click=move |_| handle_sort(col.id)>
                                    {col.header}
                                    {/* Sort indicator */}
                                </th>
                            }
                        />
                    </tr>
                </thead>
                <tbody>
                    <For
                        each=move || displayed_data.get()
                        key=|row| row.get("id").unwrap_or(&"".to_string()).clone()
                        children=|row| view! {
                            <tr>
                                <For
                                    each=move || columns.clone()
                                    key=|col| col.id
                                    children=|col| view! {
                                        <td>{row.get(col.id).unwrap_or(&"".to_string())}</td>
                                    }
                                />
                            </tr>
                        }
                    />
                </tbody>
            </table>

            {move || if paginate {
                view! { <PaginationControls /> }
            } else {
                view! {}
            }}
        </div>
    }
}

// Simple usage - component handles everything
let columns = vec![
    Column {
        id: "name",
        header: "Name",
        sortable: true,
        ..Default::default()
    },
    Column {
        id: "email",
        header: "Email",
        sortable: true,
        ..Default::default()
    },
];

view! {
    <DataTable
        data=data
        columns=columns
        sortable=true
        searchable=true
        paginate=true
        zebra=true
    />
}
```

**Pros:**
- Handles sorting, pagination, search automatically
- Less boilerplate for users
- Production-ready features
- Consistent UX

**Cons:**
- More complex implementation
- Less control over markup
- Larger bundle size

---

## 🎨 Hybrid Approach: Best of Both Worlds

**Recommendation:** Keep BOTH implementations!

### 1. Keep Existing `Table` (Basic Wrappers)

For users who need full control:

```rust
// Low-level components (existing)
pub mod table {
    pub use table::Table;
    pub use table::TableHead;
    pub use table::TableBody;
    // ... etc
}
```

**Use cases:**
- Custom table layouts
- Non-standard table structures
- Maximum flexibility needed
- Small, simple tables

---

### 2. Add New `DataTable` (Smart Component)

For users who want features out-of-the-box:

```rust
// High-level smart component (new)
pub mod data_table {
    pub use data_table::DataTable;
    pub use data_table::Column;
    pub use data_table::DataTableClasses;
    pub use data_table::DataTableTexts;
}
```

**Use cases:**
- Large datasets
- Standard data tables
- Need sorting/pagination/search
- Want minimal boilerplate

---

## 🔧 Implementation Strategy

### Phase 1: Core Data Table Component

**Create:** `src/components/data_table/`

**Files:**
- `component.rs` - Main DataTable component
- `types.rs` - Column, DataTableProps, etc.
- `header.rs` - Header with sorting
- `body.rs` - Body with loading/empty states
- `controls.rs` - Pagination controls
- `style.rs` - Enums for variants
- `mod.rs` - Module exports

**Features (MVP):**
1. ✅ Column-based data structure
2. ✅ Sorting (ascending/descending)
3. ✅ Pagination
4. ✅ Loading states
5. ✅ Empty states
6. ✅ daisyUI theme integration

**NOT in MVP:**
- URL sync (can add later)
- Search/filter (can add later)
- Custom renderers (can add later)

---

### Phase 2: Enhanced Features

**Add:**
1. Search/filtering with debounce
2. URL query synchronization
3. Custom cell renderers
4. Advanced sorting (multi-column)
5. Row selection (checkboxes)
6. Expandable rows

---

### Phase 3: Advanced Features

**Add:**
1. Virtual scrolling for huge datasets
2. Column resizing
3. Column reordering (drag-drop)
4. Export to CSV/Excel
5. Inline editing
6. Server-side pagination support

---

## 📋 Data Structure Comparison

### table-rs Approach

```rust
// Data as HashMap
let data = vec![
    hashmap! {
        "name" => "Ferris".to_string(),
        "email" => "ferris@opensass.org".to_string(),
        "age" => "5".to_string(),
    },
];

// Column definitions
let columns = vec![
    Column {
        id: "name",
        header: "Name",
        sortable: true,
        min_width: 150,
        ..Default::default()
    },
];
```

**Pros:**
- Generic - works with any data
- No need for custom structs
- Easy to work with dynamic data

**Cons:**
- No type safety
- All values must be Strings
- Runtime errors for missing keys

---

### Leptos-Friendly Approach (Proposed)

```rust
// Approach 1: HashMap (like table-rs)
let data = vec![
    hashmap! { "name" => "Ferris", "email" => "ferris@opensass.org" },
];

// Approach 2: Generic with accessor functions
#[derive(Clone)]
struct User {
    name: String,
    email: String,
    age: u32,
}

let columns = vec![
    Column::new("name", "Name")
        .accessor(|user: &User| user.name.clone())
        .sortable(true),
    Column::new("email", "Email")
        .accessor(|user: &User| user.email.clone()),
];

// Approach 3: Hybrid - type-safe with HashMap fallback
let columns = vec![
    Column {
        id: "name",
        header: "Name",
        accessor: Box::new(|row: &User| row.name.to_string()),
        sortable: true,
        ..Default::default()
    },
];
```

**Recommendation:** Start with **Approach 1** (HashMap) for compatibility with table-rs, then add **Approach 2** (Generic) as an enhancement.

---

## 🎨 Styling Integration

### How to Integrate daisyUI Styling

table-rs provides class customization via `TableClasses`. We can map these to daisyUI:

```rust
pub struct DataTableClasses {
    // Map to daisyUI classes
    pub container: &'static str,    // "overflow-x-auto"
    pub table: &'static str,        // "table table-zebra"
    pub thead: &'static str,        // ""
    pub tbody: &'static str,        // ""
    pub header_cell: &'static str,  // "cursor-pointer hover:bg-base-200"
    pub body_cell: &'static str,    // ""
    pub row: &'static str,          // "hover"
    pub loading_row: &'static str,  // ""
    pub empty_row: &'static str,    // "text-center text-base-content/60"
    pub pagination: &'static str,   // "join flex justify-center mt-4"
    pub search_input: &'static str, // "input input-bordered w-full max-w-xs mb-4"
}

impl Default for DataTableClasses {
    fn default() -> Self {
        Self {
            container: "overflow-x-auto",
            table: "table",
            thead: "",
            tbody: "",
            header_cell: "cursor-pointer select-none",
            body_cell: "",
            row: "hover",
            loading_row: "",
            empty_row: "text-center opacity-60",
            pagination: "flex justify-center gap-2 mt-4",
            search_input: "input input-bordered w-full max-w-xs mb-4",
        }
    }
}
```

### daisyUI-Specific Props

Add support for existing daisyUI table features:

```rust
#[component]
pub fn DataTable(
    // table-rs inspired props
    data: Signal<Vec<HashMap<&'static str, String>>>,
    columns: Vec<Column>,
    page_size: Option<usize>,
    loading: Signal<bool>,
    sortable: bool,
    searchable: bool,
    paginate: bool,

    // daisyUI-specific props
    #[prop(optional, into)]
    size: Signal<TableSize>,        // xs, sm, md, lg, xl

    #[prop(optional, into)]
    zebra: Signal<bool>,            // table-zebra

    #[prop(optional, into)]
    pin_rows: Signal<bool>,         // table-pin-rows

    #[prop(optional, into)]
    pin_cols: Signal<bool>,         // table-pin-cols

    // Customization
    #[prop(optional)]
    classes: DataTableClasses,

    #[prop(optional)]
    texts: DataTableTexts,
) -> impl IntoView {
    // Implementation
}
```

---

## ✅ Adaptation Plan

### Step 1: Research & Setup
- ✅ Clone table-rs repository
- ✅ Study Yew implementation
- ✅ Create comparison document
- ☐ Design Leptos API

### Step 2: Core Implementation
- ☐ Create `src/components/data_table/` directory
- ☐ Implement `types.rs` (Column, DataTableProps, etc.)
- ☐ Port core table logic from Yew to Leptos
- ☐ Implement sorting logic
- ☐ Implement pagination logic
- ☐ Add daisyUI styling

### Step 3: Sub-Components
- ☐ Implement `DataTableHeader` with sorting UI
- ☐ Implement `DataTableBody` with loading/empty states
- ☐ Implement `PaginationControls` with daisyUI Join component
- ☐ Add search input (optional)

### Step 4: Testing & Documentation
- ☐ Create demo page
- ☐ Test with small datasets (<100 rows)
- ☐ Test with large datasets (>10,000 rows)
- ☐ Test sorting performance
- ☐ Test pagination
- ☐ Write documentation
- ☐ Add to README

### Step 5: Enhancements
- ☐ Add URL sync support
- ☐ Add custom renderers
- ☐ Add row selection
- ☐ Add export functionality

---

## 🎯 API Design Proposal

### Basic Usage

```rust
use leptos::prelude::*;
use leptos_daisyui_rs::components::data_table::*;
use maplit::hashmap;

#[component]
fn UserTable() -> impl IntoView {
    let data = vec![
        hashmap! {
            "id" => "1".to_string(),
            "name" => "Ferris".to_string(),
            "email" => "ferris@rust-lang.org".to_string(),
            "role" => "Mascot".to_string(),
        },
        hashmap! {
            "id" => "2".to_string(),
            "name" => "Gopher".to_string(),
            "email" => "gopher@golang.org".to_string(),
            "role" => "Mascot".to_string(),
        },
    ];

    let columns = vec![
        Column {
            id: "name",
            header: "Name",
            sortable: true,
            min_width: 150,
            ..Default::default()
        },
        Column {
            id: "email",
            header: "Email",
            sortable: true,
            min_width: 200,
            ..Default::default()
        },
        Column {
            id: "role",
            header: "Role",
            sortable: false,
            ..Default::default()
        },
    ];

    view! {
        <DataTable
            data=Signal::derive(move || data.clone())
            columns=columns
            sortable=true
            searchable=true
            paginate=true
            page_size=10
            zebra=true
        />
    }
}
```

### Advanced Usage with Custom Styling

```rust
use leptos_daisyui_rs::components::data_table::*;

#[component]
fn AdvancedTable() -> impl IntoView {
    let data = create_rw_signal(fetch_data());
    let loading = create_rw_signal(false);

    let custom_classes = DataTableClasses {
        table: "table table-compact",
        header_cell: "bg-primary text-primary-content",
        pagination: "join flex justify-end",
        ..Default::default()
    };

    let custom_texts = DataTableTexts {
        loading: "Cargando...",
        empty: "No se encontraron resultados",
        search_placeholder: "Buscar...",
        ..Default::default()
    };

    view! {
        <DataTable
            data=data.into()
            columns=columns
            loading=loading.into()
            sortable=true
            searchable=true
            paginate=true
            page_size=20
            size=Signal::derive(|| TableSize::Small)
            zebra=true
            pin_rows=true
            classes=custom_classes
            texts=custom_texts
        />
    }
}
```

---

## 📊 Bundle Size Impact

### Estimated Size Increase

| Component | Estimated Size | Justification |
|-----------|----------------|---------------|
| Basic Table (current) | ~2KB | Simple wrappers |
| DataTable (new) | ~15-20KB | Full logic + state management |
| **Total Increase** | **~18KB** | Compressed/minified |

**Mitigation:**
- Make DataTable a separate feature flag
- Tree-shaking will remove if not used
- Users who only need basic tables won't pay the cost

```toml
# In Cargo.toml
[features]
default = ["all"]
all = ["data-table"]
data-table = []
```

---

## 🤝 Contribution to table-rs

### Opportunity

We could contribute the Leptos implementation back to table-rs!

**Benefits:**
1. Help the open-source community
2. Get their expertise/review
3. Maintain compatibility
4. Share improvements

**Process:**
1. Implement DataTable for leptos-daisyui-rs
2. Extract generic parts (remove daisyUI specifics)
3. Create PR to table-rs with Leptos support
4. Add LEPTOS.md documentation
5. Get listed on their README

---

## 🎯 Recommendation

### ✅ YES - Implement DataTable Inspired by table-rs

**Reasons:**
1. **High Value** - Adds critical missing features (sorting, pagination, search)
2. **Proven Architecture** - table-rs shows it works in production
3. **Performance** - WASM-optimized table logic is very fast
4. **User Demand** - Data tables are essential for business apps
5. **Differentiation** - Makes leptos-daisyui-rs more competitive
6. **Best of Both** - Keep simple Table for flexibility, add DataTable for features

### 📋 Implementation Priority

**Priority:** P1 (High)
**Estimated Effort:** 2-3 weeks
**Dependencies:** None (can start immediately)

### 🗺️ Roadmap

**Week 1:**
- Design Leptos API
- Implement core types and structures
- Port sorting logic

**Week 2:**
- Implement pagination
- Add loading/empty states
- Integrate daisyUI styling

**Week 3:**
- Add search/filtering
- Create demo page
- Write documentation
- Polish and test

---

## 📚 Reference Links

- **table-rs GitHub:** https://github.com/opensass/table-rs
- **table-rs Yew Demo:** https://table-rs.netlify.app
- **table-rs Dioxus Demo:** https://table-dio.netlify.app
- **daisyUI Table Docs:** https://daisyui.com/components/table/

---

## 🎉 Conclusion

The opensass/table-rs library provides an **excellent foundation** for a production-ready data table component. By adapting their proven architecture to Leptos with daisyUI styling, we can provide users with a best-in-class table component that:

- ✅ Handles large datasets efficiently (WASM performance)
- ✅ Provides sorting, pagination, and search out of the box
- ✅ Integrates seamlessly with daisyUI themes
- ✅ Maintains the flexibility of the basic Table for custom use cases
- ✅ Follows Leptos best practices with reactive Signals
- ✅ Offers excellent developer experience with minimal boilerplate

**This is a high-value addition to leptos-daisyui-rs that will make it more competitive with other UI libraries.**

---

**Ready to implement!** 🚀

Shall we create a beads issue and start building the DataTable component?
