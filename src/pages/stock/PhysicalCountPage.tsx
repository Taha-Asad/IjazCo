import { useState } from "react";
import {
  Stack,
  Card,
  Title,
  Text,
  Group,
  NumberInput,
  Button,
  Textarea,
  Select,
  Alert,
  SimpleGrid,
} from "@mantine/core";
import { useForm } from "@mantine/form";
import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { notifications } from "@mantine/notifications";
import { IconClipboardCheck, IconAlertCircle } from "@tabler/icons-react";
import { PageHeader } from "../../components/common/PageHeader";
import { stockApi } from "../../api/stock";
import { inventoryApi } from "../../api/inventory";

export function PhysicalCountPage() {
  const queryClient = useQueryClient();

  const { data: inventoryData } = useQuery({
    queryKey: ["inventory-select"],
    queryFn: () => inventoryApi.list({ per_page: 500 }),
  });

  const form = useForm({
    initialValues: {
      item_id: "",
      branch_id: "",
      counted_quantity: 0,
      notes: "",
    },
    validate: {
      item_id: (v) => (!v ? "Item required" : null),
      branch_id: (v) => (!v ? "Branch required" : null),
      counted_quantity: (v) => (v < 0 ? "Must be >= 0" : null),
    },
  });

  const mutation = useMutation({
    mutationFn: stockApi.physicalCount,
    onSuccess: () => {
      notifications.show({
        title: "Count Recorded",
        message: "Physical count has been recorded.",
        color: "green",
      });
      queryClient.invalidateQueries({ queryKey: ["stock"] });
      form.reset();
    },
    onError: () => {
      notifications.show({
        title: "Error",
        message: "Failed to record count.",
        color: "red",
      });
    },
  });

  const itemOptions =
    inventoryData?.data?.map((i) => ({
      value: i.id,
      label: `${i.sku} - ${i.name}`,
    })) || [];

  return (
    <Stack>
      <PageHeader
        title="Physical Count"
        description="Record actual stock quantities from physical inventory count"
        breadcrumbs={[
          { label: "Home", path: "/" },
          { label: "Stock", path: "/stock" },
          { label: "Physical Count" },
        ]}
      />

      <Alert icon={<IconAlertCircle />} color="blue" radius="md">
        Physical count will override the system quantity. Make sure counts are
        accurate.
      </Alert>

      <Card withBorder radius="md" p="lg" maw={600}>
        <Title order={4} mb="md">
          <Group gap="xs">
            <IconClipboardCheck size={20} />
            Record Physical Count
          </Group>
        </Title>
        <form onSubmit={form.onSubmit((v) => mutation.mutate(v))}>
          <Stack>
            <Select
              label="Item"
              placeholder="Select item"
              data={itemOptions}
              searchable
              required
              {...form.getInputProps("item_id")}
            />
            <NumberInput
              label="Counted Quantity"
              description="Enter the actual quantity you counted"
              min={0}
              required
              {...form.getInputProps("counted_quantity")}
            />
            <Textarea
              label="Notes"
              placeholder="Any relevant notes about this count..."
              rows={3}
              {...form.getInputProps("notes")}
            />
            <Button
              type="submit"
              loading={mutation.isPending}
              leftSection={<IconClipboardCheck size={16} />}
            >
              Record Count
            </Button>
          </Stack>
        </form>
      </Card>
    </Stack>
  );
}
