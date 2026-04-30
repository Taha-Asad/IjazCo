import {
  DataTable as MantineDataTable,
  DataTableColumn,
} from "mantine-datatable";
import { EmptyState } from "./EmptyState";

interface DataTableProps<T> {
  records: T[];
  columns: DataTableColumn<T>[];
  loading?: boolean;
  totalRecords?: number;
  page?: number;
  pageSize?: number;
  onPageChange?: (page: number) => void;
  minHeight?: number;
  emptyState?: {
    title?: string;
    description?: string;
  };
  onRowClick?: (record: T) => void;
}

export function DataTable<T extends { id: string }>({
  records,
  columns,
  loading = false,
  totalRecords = 0,
  page = 1,
  pageSize = 20,
  onPageChange,
  minHeight = 400,
  emptyState,
  onRowClick,
}: DataTableProps<T>) {
  return (
    <MantineDataTable
      records={records}
      columns={columns}
      fetching={loading}
      totalRecords={totalRecords}
      recordsPerPage={pageSize}
      page={page}
      onPageChange={onPageChange || (() => {})}
      minHeight={minHeight}
      noRecordsText={emptyState?.title || "No records found"}
      highlightOnHover={!!onRowClick}
      onRowClick={onRowClick ? ({ record }) => onRowClick(record) : undefined}
      withTableBorder
      borderRadius="md"
      striped
      shadow="sm"
      styles={{
        table: { tableLayout: "fixed" },
      }}
    />
  );
}
