//! # Table Component
//!
//! The Table component provides a comprehensive set of styled table elements for displaying
//! tabular data. It includes semantic table components with built-in styling options and
//! responsive features.
//!
//! ## Components
//!
//! - [`Table`] - Main table container with size and layout options
//! - [`TableHead`] - Table header section (`<thead>`)
//! - [`TableBody`] - Table body section (`<tbody>`)
//! - [`TableFoot`] - Table footer section (`<tfoot>`)
//! - [`TableRow`] - Table row (`<tr>`)
//! - [`TableHeader`] - Table header cell (`<th>`)
//! - [`TableCell`] - Table data cell (`<td>`)
//!
//! ## Features
//!
//! - **Size Variants**: Extra small, small, medium, and large sizing
//! - **Zebra Striping**: Alternating row backgrounds for better readability
//! - **Pinned Elements**: Pin rows or columns for scrollable tables
//! - **Semantic Structure**: Proper HTML table semantics
//! - **Responsive Design**: Works well on different screen sizes
//!
//! ## CSS Classes
//!
//! | Component | CSS Class | Description |
//! |-----------|-----------|-------------|
//! | `Table` | `table` | Base table styling |
//! | `Table` (xs) | `table-xs` | Extra small size |
//! | `Table` (sm) | `table-sm` | Small size |
//! | `Table` (md) | `table-md` | Medium size |
//! | `Table` (lg) | `table-lg` | Large size |
//! | `Table` (zebra) | `table-zebra` | Zebra striping |
//! | `Table` (pin rows) | `table-pin-rows` | Pin header/footer rows |
//! | `Table` (pin cols) | `table-pin-cols` | Pin first column |
//!
//! ## Usage
//!
//! ### Basic Table
//!
//! ```rust
//! use leptos::prelude::*;
//! use leptos_daisyui::*;
//!
//! #[component]
//! fn App() -> impl IntoView {
//!     view! {
//!         <Table>
//!             <TableHead>
//!                 <TableRow>
//!                     <TableHeader>"Name"</TableHeader>
//!                     <TableHeader>"Job"</TableHeader>
//!                     <TableHeader>"Company"</TableHeader>
//!                 </TableRow>
//!             </TableHead>
//!             <TableBody>
//!                 <TableRow>
//!                     <TableCell>"Cy Ganderton"</TableCell>
//!                     <TableCell>"Quality Control Specialist"</TableCell>
//!                     <TableCell>"Littel, Schaden and Vandervort"</TableCell>
//!                 </TableRow>
//!                 <TableRow>
//!                     <TableCell>"Hart Hagerty"</TableCell>
//!                     <TableCell>"Desktop Support Technician"</TableCell>
//!                     <TableCell>"Zemlak, Daniel and Leannon"</TableCell>
//!                 </TableRow>
//!             </TableBody>
//!         </Table>
//!     }
//! }
//! ```
//!
//! ### With Zebra Striping and Size
//!
//! ```rust
//! use leptos::prelude::*;
//! use leptos_daisyui::*;
//!
//! #[component]
//! fn App() -> impl IntoView {
//!     let zebra_enabled = RwSignal::new(true);
//!
//!     view! {
//!         <Table size=TableSize::Sm zebra=zebra_enabled>
//!             <TableHead>
//!                 <TableRow>
//!                     <TableHeader>"ID"</TableHeader>
//!                     <TableHeader>"Product"</TableHeader>
//!                     <TableHeader>"Price"</TableHeader>
//!                 </TableRow>
//!             </TableHead>
//!             <TableBody>
//!                 <TableRow>
//!                     <TableCell>"001"</TableCell>
//!                     <TableCell>"MacBook Pro"</TableCell>
//!                     <TableCell>"$1,999"</TableCell>
//!                 </TableRow>
//!                 <TableRow>
//!                     <TableCell>"002"</TableCell>
//!                     <TableCell>"iPhone 15"</TableCell>
//!                     <TableCell>"$999"</TableCell>
//!                 </TableRow>
//!             </TableBody>
//!         </Table>
//!     }
//! }
//! ```
//!
//! ### With Pinned Rows and Columns
//!
//! ```rust
//! use leptos::prelude::*;
//! use leptos_daisyui::*;
//!
//! #[component]
//! fn App() -> impl IntoView {
//!     let pin_rows = RwSignal::new(true);
//!     let pin_cols = RwSignal::new(true);
//!
//!     view! {
//!         <div class="overflow-x-auto max-w-4xl">
//!             <Table pin_rows=pin_rows pin_cols=pin_cols>
//!                 <TableHead>
//!                     <TableRow>
//!                         <TableHeader>"Name"</TableHeader>
//!                         <TableHeader>"Jan"</TableHeader>
//!                         <TableHeader>"Feb"</TableHeader>
//!                         <TableHeader>"Mar"</TableHeader>
//!                         // ... more columns
//!                     </TableRow>
//!                 </TableHead>
//!                 <TableBody>
//!                     // ... table data
//!                 </TableBody>
//!             </Table>
//!         </div>
//!     }
//! }
//! ```
//!
//! ## Accessibility
//!
//! - Use proper table structure with thead, tbody, and tfoot
//! - Include descriptive headers for all columns
//! - Consider adding `scope` attributes for complex tables
//! - Provide table captions or summaries when helpful
//! - Ensure sufficient color contrast for all text
//! - Test with keyboard navigation and screen readers
//!
//! ## Best Practices
//!
//! - Keep table headers clear and concise
//! - Use consistent data formatting within columns
//! - Consider pagination for large datasets
//! - Implement sorting and filtering for better usability
//! - Use zebra striping for tables with many rows
//! - Test responsive behavior on mobile devices
//! - Consider pinning important columns in wide tables

mod component;
mod style;

pub use component::*;
pub use style::*;
