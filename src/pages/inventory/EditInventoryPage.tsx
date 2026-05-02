import { useNavigate, useParams } from "react-router-dom";
import { Stack } from "@mantine/core";
import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { notifications } from "@mantine/notifications";
import { PageHeader } from "../../components/common/PageHeader";
import { InventoryItemForm } from "../../components/forms/InventoryItemForm";
import { inventoryApi } from "../../api/inventory";

export function EditInventoryPage() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const queryClient = useQueryClient();

  const { data: itemData, isLoading } = useQuery({
    queryKey: ["inventory", id],
    queryFn: () => inventoryApi.getById(id!),
    enabled: !!id,
  });

  const updateMutation = useMutation({
    mutationFn: (values: any) => inventoryApi.update(id!, values),
    onSuccess: () => {
      notifications.show({
        title: "Updated",
        message: "Inventory item updated successfully.",
        color: "green",
      });
      queryClient.invalidateQueries({ queryKey: ["inventory"] });
      queryClient.invalidateQueries({ queryKey: ["inventory", id] });
      navigate(`/inventory/${id}`);
    },
    onError: (error: any) => {
      notifications.show({
        title: "Error",
        message: error?.response?.data?.message || "Failed to update item",
        color: "red",
      });
    },
  });

  return (
    <Stack>
      <PageHeader
        title="Edit Inventory Item"
        description="Update inventory item details"
        breadcrumbs={[
          { label: "Home", path: "/" },
          { label: "Inventory", path: "/inventory" },
          { label: "Edit" },
        ]}
      />
      {!isLoading && itemData?.data && (
        <InventoryItemForm
          initialValues={itemData.data}
          onSubmit={async (values) => {
            await updateMutation.mutateAsync(values);
          }}
          loading={updateMutation.isPending}
        />
      )}
    </Stack>
  );
}
