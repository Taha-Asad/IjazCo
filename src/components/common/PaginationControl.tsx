import { Group, Select, Text, Pagination } from "@mantine/core";

interface PaginationControlProps {
  page: number;
  pageSize: number;
  totalRecords: number;
  onPageChange: (page: number) => void;
  onPageSizeChange?: (size: number) => void;
  pageSizeOptions?: number[];
}

export function PaginationControl({
  page,
  pageSize,
  totalRecords,
  onPageChange,
  onPageSizeChange,
  pageSizeOptions = [10, 20, 50, 100],
}: PaginationControlProps) {
  const totalPages = Math.ceil(totalRecords / pageSize);
  const from = Math.min((page - 1) * pageSize + 1, totalRecords);
  const to = Math.min(page * pageSize, totalRecords);

  return (
    <Group justify="space-between" mt="md">
      <Text size="sm" c="dimmed">
        Showing {from}–{to} of {totalRecords} records
      </Text>
      <Group gap="sm">
        {onPageSizeChange && (
          <Group gap="xs">
            <Text size="sm" c="dimmed">
              Rows:
            </Text>
            <Select
              size="xs"
              w={70}
              value={String(pageSize)}
              onChange={(v) => v && onPageSizeChange(Number(v))}
              data={pageSizeOptions.map((s) => ({
                value: String(s),
                label: String(s),
              }))}
            />
          </Group>
        )}
        <Pagination
          value={page}
          onChange={onPageChange}
          total={totalPages}
          size="sm"
          radius="md"
        />
      </Group>
    </Group>
  );
}
