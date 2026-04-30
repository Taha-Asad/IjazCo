import { Stack, Card } from "@mantine/core";
import { useNavigate } from "react-router-dom";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { notifications } from "@mantine/notifications";
import { PageHeader } from "../../components/common/PageHeader";
import { SalesInvoiceForm } from "../../components/forms/SalesInvoiceForm";
import { salesApi } from "../../api/sales";

export function CreateInvoicePage() {
  const navigate = useNavigate();
  const queryClient = useQueryClient();

  const mutation = useMutation({
    mutationFn: salesApi.create,
    onSuccess: (res) => {
      notifications.show({
        title: "Created",
        message: "Invoice created.",
        color: "green",
      });
      queryClient.invalidateQueries({ queryKey: ["sales"] });
      navigate(`/sales/${res.data.id}`);
    },
    onError: (err: any) => {
      notifications.show({
        title: "Error",
        message: err?.response?.data?.message || "Failed to create invoice.",
        color: "red",
      });
    },
  });

  return (
    <Stack>
      <PageHeader
        title="Create Invoice"
        description="Create a new sales invoice"
        breadcrumbs={[
          { label: "Home", path: "/" },
          { label: "Sales", path: "/sales" },
          { label: "Create" },
        ]}
      />
      <Card withBorder radius="md" p="lg">
        <SalesInvoiceForm
          onSubmit={async (v) => {
            await mutation.mutateAsync(v);
          }}
          loading={mutation.isPending}
        />
      </Card>
    </Stack>
  );
}
