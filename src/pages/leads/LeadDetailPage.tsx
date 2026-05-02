import { useNavigate, useParams } from "react-router-dom";
import { Stack, Group, Badge, Text, Button, ActionIcon } from "@mantine/core";
import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { notifications } from "@mantine/notifications";
import { IconEdit, IconTrash, IconArrowLeft } from "@tabler/icons-react";
import { PageHeader } from "../../components/common/PageHeader";
import { openConfirmModal } from "../../components/common/ConfirmModal";
import { leadsApi } from "../../api/leads";
import { LeadForm } from "../../components/forms/LeadForm";

const STATUS_COLORS: Record<string, string> = {
  new: "blue",
  contacted: "cyan",
  qualified: "indigo",
  proposal: "violet",
  negotiation: "orange",
  won: "green",
  lost: "red",
};

export function LeadDetailPage() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const queryClient = useQueryClient();

  const { data: leadData, isLoading } = useQuery({
    queryKey: ["leads", id],
    queryFn: () => leadsApi.getById(id!),
    enabled: !!id,
  });

  const deleteMutation = useMutation({
    mutationFn: () => leadsApi.delete(id!),
    onSuccess: () => {
      notifications.show({
        title: "Deleted",
        message: "Lead deleted successfully.",
        color: "green",
      });
      queryClient.invalidateQueries({ queryKey: ["leads"] });
      navigate("/leads");
    },
  });

  const lead = leadData?.data;

  if (isLoading || !lead) {
    return <div>Loading...</div>;
  }

  return (
    <Stack>
      <PageHeader
        title={`Lead: ${lead.name}`}
        description={`Lead #${lead.lead_number}`}
        breadcrumbs={[
          { label: "Home", path: "/" },
          { label: "Leads", path: "/leads" },
          { label: lead.name },
        ]}
        action={{
          label: "Back to Leads",
          icon: <IconArrowLeft size={16} />,
          onClick: () => navigate("/leads"),
        }}
      />

      <Group justify="space-between">
        <Badge color={STATUS_COLORS[lead.status] || "gray"} variant="light" size="lg">
          {lead.status}
        </Badge>
        <Group>
          <Button
            leftSection={<IconEdit size={16} />}
            onClick={() => navigate(`/leads/${id}/edit`)}
          >
            Edit
          </Button>
          <Button
            color="red"
            leftSection={<IconTrash size={16} />}
            onClick={() =>
              openConfirmModal({
                title: "Delete Lead",
                message: `Are you sure you want to delete lead "${lead.name}"?`,
                danger: true,
                onConfirm: () => deleteMutation.mutate(),
              })
            }
          >
            Delete
          </Button>
        </Group>
      </Group>

      <Stack gap="xs">
        <Text><strong>Name:</strong> {lead.name}</Text>
        {lead.company_name && <Text><strong>Company:</strong> {lead.company_name}</Text>}
        {lead.email && <Text><strong>Email:</strong> {lead.email}</Text>}
        {lead.phone && <Text><strong>Phone:</strong> {lead.phone}</Text>}
        {lead.estimated_value && (
          <Text><strong>Estimated Value:</strong> ${lead.estimated_value.toLocaleString()}</Text>
        )}
        {lead.source && <Text><strong>Source:</strong> {lead.source}</Text>}
        {lead.description && <Text><strong>Description:</strong> {lead.description}</Text>}
      </Stack>
    </Stack>
  );
}

export function EditLeadPage() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const queryClient = useQueryClient();

  const { data: leadData, isLoading } = useQuery({
    queryKey: ["leads", id],
    queryFn: () => leadsApi.getById(id!),
    enabled: !!id,
  });

  const updateMutation = useMutation({
    mutationFn: (values: any) => leadsApi.update(id!, values),
    onSuccess: () => {
      notifications.show({
        title: "Updated",
        message: "Lead updated successfully.",
        color: "green",
      });
      queryClient.invalidateQueries({ queryKey: ["leads"] });
      queryClient.invalidateQueries({ queryKey: ["leads", id] });
      navigate(`/leads/${id}`);
    },
  });

  const lead = leadData?.data;

  if (isLoading || !lead) {
    return <div>Loading...</div>;
  }

  return (
    <Stack>
      <PageHeader
        title="Edit Lead"
        description={`Edit lead: ${lead.name}`}
        breadcrumbs={[
          { label: "Home", path: "/" },
          { label: "Leads", path: "/leads" },
          { label: "Edit" },
        ]}
      />
      <LeadForm
        initialValues={lead}
        onSubmit={async (values) => {
          await updateMutation.mutateAsync(values);
        }}
        loading={updateMutation.isPending}
      />
    </Stack>
  );
}
