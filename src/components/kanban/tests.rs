#[cfg(test)]
mod tests {
    use super::super::types::*;

    #[test]
    fn test_module_compiles() {
        // Basic smoke test to ensure module compiles
        let _column = KanbanColumn::new("col-1", "To Do");
        let _card = KanbanCard::new("card-1", "Task");
        let _filters = KanbanFilters::new();
    }
}
