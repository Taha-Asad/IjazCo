import {
  Select,
  NumberInput,
  Button,
  Stack,
  Group,
  ActionIcon,
  Paper,
  Text,
  Divider,
  Textarea,
} from "@mantine/core";
import { DatePickerInput } from "@mantine/dates";
import { useForm } from "@mantine/form";
import { useState } from "react";
import { useQuery } from "@tanstack/react-query";
import { IconPlus, IconTrash } from "@tabler/icons-react";
import { suppliersApi } from "../../api/suppliers";
import { inventoryApi } from "../../api/inventory";
import { formatCurrency } from "../../utils/formatters";

interface POLineItem {
  item_id: string;
  quantity: number;
  unit_price: number;
  total: number;
}

interface PurchaseOrderFormProps {
  onSubmit: (values: any) => Promise<void>;
  loading?: boolean;
  initialValues?: any;
}

export function PurchaseOrderForm({
  onSubmit,
  loading,
  initialValues,
}: PurchaseOrderFormProps) {
  const [items, setItems] = useState<POLineItem[]>(initialValues?.items || []);

  const { data: suppliersData } = useQuery({
    queryKey: ["suppliers-select"],
    queryFn: () => suppliersApi.list({ per_page: 200 }),
  });

  const { data: inventoryData } = useQuery({
    queryKey: ["inventory-select"],
    queryFn: () => inventoryApi.list({ per_page: 500 }),
  });

  const form = useForm({
    initialValues: {
      supplier_id: initialValues?.supplier_id || "",
      branch_id: initialValues?.branch_id || "",
      expected_date: initialValues?.expected_date
        ? new Date(initialValues.expected_date)
        : (null as Date | null),
      notes: initialValues?.notes || "",
    },
    validate: {
      supplier_id: (v) => (!v ? "Supplier required" : null),
    },
  });

  const supplierOptions =
    suppliersData?.data?.map((s) => ({
      value: s.id,
      label: s.name,
    })) || [];

  const inventoryOptions =
    inventoryData?.data?.map((i) => ({
      value: i.id,
      label: `${i.sku} - ${i.name}`,
      price: i.cost_price,
    })) || [];

  const addItem = () =>
    setItems((p) => [
      ...p,
      { item_id: "", quantity: 1, unit_price: 0, total: 0 },
    ]);

  const removeItem = (i: number) =>
    setItems((p) => p.filter((_, idx) => idx !== i));

  const updateItem = (index: number, field: string, value: any) => {
    setItems((prev) =>
      prev.map((item, i) => {
        if (i !== index) return item;
        const updated = { ...item, [field]: value };
        if (field === "item_id") {
          const found = inventoryOptions.find((o) => o.value === value);
          updated.unit_price = found?.price || 0;
        }
        updated.total = updated.quantity * updated.unit_price;
        return updated;
      }),
    );
  };

  const total = items.reduce((s, i) => s + i.total, 0);

  const handleSubmit = async (values: typeof form.values) => {
    await onSubmit({
      ...values,
      items: items.map(({ item_id, quantity, unit_price }) => ({
        item_id,
        quantity,
        unit_price,
      })),
    });
  };

  return (
    <form onSubmit={form.onSubmit(handleSubmit)}>
      <Stack>
        <Group grow>
          <Select
            label="Supplier"
            placeholder="Select supplier"
            data={supplierOptions}
            required
            searchable
            {...form.getInputProps("supplier_id")}
          />
          <DatePickerInput
            label="Expected Delivery"
            valueFormat="MMM DD, YYYY"
            {...form.getInputProps("expected_date")}
          />
        </Group>

        <Divider label="Order Items" />

        {items.map((item, index) => (
          <Paper key={index} withBorder p="sm" radius="md">
            <Group align="flex-end">
              <Select
                label="Item"
                data={inventoryOptions}
                searchable
                style={{ flex: 2 }}
                value={item.item_id}
                onChange={(v) => v && updateItem(index, "item_id", v)}
              />
              <NumberInput
                label="Qty"
                min={1}
                style={{ width: 80 }}
                value={item.quantity}
                onChange={(v) => updateItem(index, "quantity", Number(v))}
              />
              <NumberInput
                label="Unit Cost"
                prefix="$"
                decimalScale={2}
                style={{ width: 120 }}
                value={item.unit_price}
                onChange={(v) => updateItem(index, "unit_price", Number(v))}
              />
              <Text fw={600} size="sm" mb={6}>
                {formatCurrency(item.total)}
              </Text>
              <ActionIcon
                color="red"
                variant="subtle"
                mb={4}
                onClick={() => removeItem(index)}
              >
                <IconTrash size={16} />
              </ActionIcon>
            </Group>
          </Paper>
        ))}

        <Button
          variant="light"
          leftSection={<IconPlus size={16} />}
          onClick={addItem}
        >
          Add Item
        </Button>

        <Divider />

        <Group justify="flex-end">
          <Text fw={700}>Total: {formatCurrency(total)}</Text>
        </Group>

        <Textarea label="Notes" rows={3} {...form.getInputProps("notes")} />

        <Button type="submit" loading={loading} disabled={items.length === 0}>
          {initialValues ? "Save Changes" : "Create Purchase Order"}
        </Button>
      </Stack>
    </form>
  );
}
