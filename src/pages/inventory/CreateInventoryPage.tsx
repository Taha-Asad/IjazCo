import { useNavigate } from "react-router-dom";
import { Stack } from "@mantine/core";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { notifications } from "@mantine/notifications";
import { PageHeader } from "../../components/common/PageHeader";
import { InventoryItemForm } from "../../components/forms/InventoryItemForm";
import { inventoryApi } from "../../api/inventory";
import { useAuthStore } from "../../store/authStore";

export function CreateInventoryPage() {
  const navigate = useNavigate();
  const queryClient = useQueryClient();
  const { user } = useAuthStore();

  const createMutation = useMutation({
    mutationFn: (values: any) =>
      inventoryApi.create({ ...values, company_id: user?.company_id }),
    onSuccess: (res) => {
      notifications.show({
        title: "Created",
        message: "Inventory item created successfully.",
        color: "green",
      });
      queryClient.invalidateQueries({ queryKey: ["inventory"] });
      navigate(`/inventory/${res.id}`);
    },
    onError: (error: any) => {
      notifications.show({
        title: "Error",
        message: error?.response?.data?.message || "Failed to create item",
        color: "red",
      });
    },
  });

  return (
    <Stack>
      <PageHeader
        title="Create Inventory Item"
        description="Add a new item to your inventory"
        breadcrumbs={[
          { label: "Home", path: "/" },
          { label: "Inventory", path: "/inventory" },
          { label: "Create" },
        ]}
      />
      <InventoryItemForm
        onSubmit={async (values) => {
          await createMutation.mutateAsync(values);
        }}
        loading={createMutation.isPending}
      />
    </Stack>
  );
}
