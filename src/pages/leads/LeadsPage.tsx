import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { Stack, Group, Badge, ActionIcon, Tooltip, Modal, Select } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { DataTable } from "mantine-datatable";
import { notifications } from "@mantine/notifications";
import { IconPlus, IconEye, IconTrash } from "@tabler/icons-react";
import { PageHeader } from "../../components/common/PageHeader";
import { SearchInput } from "../../components/common/SearchInput";
import { openConfirmModal } from "../../components/common/ConfirmModal";
import { LeadForm } from "../../components/forms/LeadForm";
import { leadsApi, type Lead } from "../../api/leads";
import { useDebounce } from "../../hooks/useDebounce";

const PAGE_SIZE = 20;

const STATUS_COLORS: Record<string, string> = {
  new: "blue",
  contacted: "cyan",
  qualified: "indigo",
  proposal: "violet",
  negotiation: "orange",
  won: "green",
  lost: "red",
};

export function LeadsPage() {
  const navigate = useNavigate();
  const queryClient = useQueryClient();
  const [page, setPage] = useState(1);
  const [search, setSearch] = useState("");
  const [statusFilter, setStatusFilter] = useState<string | null>(null);
  const [opened, { open, close }] = useDisclosure(false);
  const debouncedSearch = useDebounce(search, 400);

  const { data, isLoading } = useQuery({
    queryKey: ["leads", page, debouncedSearch, statusFilter],
    queryFn: () =>
      leadsApi.list({
        page: Number(page),
        per_page: Number(PAGE_SIZE),
        ...(debouncedSearch?.trim() && { search: debouncedSearch }),
        ...(statusFilter && { status: statusFilter }),
      }),
  });

  const createMutation = useMutation({
    mutationFn: (v: any) => {
      // Clean up empty strings to undefined for optional fields
      const data = {
        ...v,
        email: v.email || undefined,
        phone: v.phone || undefined,
        company_name: v.company_name || undefined,
        status: v.status || undefined,
        source: v.source || undefined,
        estimated_value: v.estimated_value || undefined,
        description: v.description || undefined,
        expected_close_date: v.expected_close_date || undefined,
      };
      return leadsApi.create(data);
    },
    onSuccess: (res) => {
      notifications.show({
        title: "Created",
        message: "Lead created successfully.",
        color: "green",
      });
      queryClient.invalidateQueries({ queryKey: ["leads"] });
      close();
      navigate(`/leads/${res.data.id}`);
    },
    onError: (error: any) => {
      notifications.show({
        title: "Error",
        message: error?.response?.data?.message || "Failed to create lead",
        color: "red",
      });
    },
  });

  const deleteMutation = useMutation({
    mutationFn: (id: string) => leadsApi.delete(id),
    onSuccess: () => {
      notifications.show({
        title: "Deleted",
        message: "Lead deleted successfully.",
        color: "green",
      });
      queryClient.invalidateQueries({ queryKey: ["leads"] });
    },
  });

  return (
    <Stack>
      <PageHeader
        title="Leads"
        description="Manage your sales leads"
        breadcrumbs={[{ label: "Home", path: "/" }, { label: "Leads" }]}
        action={{
          label: "Add Lead",
          icon: <IconPlus size={16} />,
          onClick: open,
        }}
      />
      <Group>
        <SearchInput value={search} onChange={setSearch} w={280} />
        <Select
          placeholder="Filter by status"
          clearable
          data={[
            { value: "new", label: "New" },
            { value: "contacted", label: "Contacted" },
            { value: "qualified", label: "Qualified" },
            { value: "proposal", label: "Proposal" },
            { value: "negotiation", label: "Negotiation" },
            { value: "won", label: "Won" },
            { value: "lost", label: "Lost" },
          ]}
          value={statusFilter}
          onChange={(v) => setStatusFilter(v)}
          w={200}
        />
      </Group>
      <DataTable
        records={data?.data || []}
        fetching={isLoading}
        totalRecords={data?.total_items || 0}
        recordsPerPage={PAGE_SIZE}
        page={page}
        onPageChange={setPage}
        columns={[
          { accessor: "lead_number", title: "Lead #", width: 120 },
          { accessor: "name", title: "Name" },
          { accessor: "company_name", title: "Company" },
          { accessor: "email", title: "Email" },
          {
            accessor: "status",
            title: "Status",
            width: 120,
            render: (lead: Lead) => (
              <Badge color={STATUS_COLORS[lead.status] || "gray"} variant="light">
                {lead.status}
              </Badge>
            ),
          },
          {
            accessor: "estimated_value",
            title: "Value",
            render: (lead: Lead) =>
              lead.estimated_value
                ? `$${lead.estimated_value.toLocaleString()}`
                : "—",
          },
          {
            accessor: "actions",
            title: "",
            width: 80,
            render: (lead: Lead) => (
              <Group gap="xs" justify="flex-end">
                <Tooltip label="View">
                  <ActionIcon
                    variant="subtle"
                    onClick={() => navigate(`/leads/${lead.id}`)}
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
                        title: "Delete Lead",
                        message: `Delete lead "${lead.name}"?`,
                        danger: true,
                        onConfirm: () => deleteMutation.mutate(lead.id),
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

      <Modal opened={opened} onClose={close} title="Create Lead" size="md">
        <LeadForm
          onSubmit={async (v) => {
            await createMutation.mutateAsync(v);
          }}
          loading={createMutation.isPending}
        />
      </Modal>
    </Stack>
  );
}
