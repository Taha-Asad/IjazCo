import { useState } from "react";
import { Stack, Group, ActionIcon, Tooltip, Badge, Modal } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { DataTable } from "mantine-datatable";
import { notifications } from "@mantine/notifications";
import { IconPlus, IconTrash, IconEye } from "@tabler/icons-react";
import { useNavigate } from "react-router-dom";
import { PageHeader } from "../../components/common/PageHeader";
import { SearchInput } from "../../components/common/SearchInput";
import { openConfirmModal } from "../../components/common/ConfirmModal";
import { CompanyForm } from "../../components/forms/CompanyForm";
import { companiesApi } from "../../api/companies";
import { useDebounce } from "../../hooks/useDebounce";
import { formatDate } from "../../utils/formatters";

const PAGE_SIZE = 20;

export function CompaniesPage() {
  const navigate = useNavigate();
  const queryClient = useQueryClient();
  const [page, setPage] = useState(1);
  const [search, setSearch] = useState("");
  const [opened, { open, close }] = useDisclosure(false);
  const debouncedSearch = useDebounce(search, 400);

  const { data, isLoading } = useQuery({
    queryKey: ["companies", page, debouncedSearch],
    queryFn: () =>
      companiesApi.list({ page, per_page: PAGE_SIZE, search: debouncedSearch }),
  });

  const createMutation = useMutation({
    mutationFn: companiesApi.create,
    onSuccess: () => {
      notifications.show({
        title: "Created",
        message: "Company created.",
        color: "green",
      });
      queryClient.invalidateQueries({ queryKey: ["companies"] });
      close();
    },
  });

  const deleteMutation = useMutation({
    mutationFn: (id: string) => companiesApi.delete(id),
    onSuccess: () => {
      notifications.show({
        title: "Deleted",
        message: "Company deleted.",
        color: "green",
      });
      queryClient.invalidateQueries({ queryKey: ["companies"] });
    },
  });

  return (
    <Stack>
      <PageHeader
        title="Companies"
        description="Manage organizations in the system"
        breadcrumbs={[{ label: "Home", path: "/" }, { label: "Companies" }]}
        action={{
          label: "Add Company",
          icon: <IconPlus size={16} />,
          onClick: open,
        }}
      />

      <SearchInput value={search} onChange={setSearch} w={300} />

      <DataTable
        records={data?.data || []}
        fetching={isLoading}
        totalRecords={data?.total_items || 0}
        recordsPerPage={PAGE_SIZE}
        page={page}
        onPageChange={setPage}
        columns={[
          { accessor: "name", title: "Company Name" },
          { accessor: "email", title: "Email" },
          { accessor: "city", title: "City" },
          { accessor: "country", title: "Country" },
          {
            accessor: "is_active",
            title: "Status",
            render: (c) => (
              <Badge color={c.is_active ? "green" : "gray"} variant="light">
                {c.is_active ? "Active" : "Inactive"}
              </Badge>
            ),
          },
          {
            accessor: "created_at",
            title: "Created",
            render: (c) => formatDate(c.created_at),
          },
          {
            accessor: "actions",
            title: "",
            width: 100,
            render: (c) => (
              <Group gap="xs" justify="flex-end">
                <Tooltip label="View">
                  <ActionIcon
                    variant="subtle"
                    onClick={() => navigate(`/companies/${c.id}`)}
                  >
                    <IconEye size={16} />
                  </ActionIcon>
                </Tooltip>
                <Tooltip label="Delete">
                  <ActionIcon
                    variant="subtle"
                    color="red"
                    onClick={() =>
                      openConfirmModal({
                        title: "Delete Company",
                        message: `Delete "${c.name}"?`,
                        danger: true,
                        onConfirm: () => deleteMutation.mutate(c.id),
                      })
                    }
                  >
                    <IconTrash size={16} />
                  </ActionIcon>
                </Tooltip>
              </Group>
            ),
          },
        ]}
        highlightOnHover
        withTableBorder
        borderRadius="md"
        striped
      />

      <Modal opened={opened} onClose={close} title="Create Company" size="md">
        <CompanyForm
          onSubmit={async (v) => {
            await createMutation.mutateAsync(v);
          }}
          loading={createMutation.isPending}
        />
      </Modal>
    </Stack>
  );
}
