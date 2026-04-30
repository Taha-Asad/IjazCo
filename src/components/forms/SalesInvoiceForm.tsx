import {
  TextInput,
  NumberInput,
  Select,
  Button,
  Stack,
  Group,
  ActionIcon,
  Table,
  Text,
  Divider,
  Paper,
  Textarea,
} from "@mantine/core";
import { DatePickerInput } from "@mantine/dates";
import { useForm } from "@mantine/form";
import { useState } from "react";
import { useQuery } from "@tanstack/react-query";
import { IconPlus, IconTrash } from "@tabler/icons-react";
import { customersApi } from "../../api/customers";
import { inventoryApi } from "../../api/inventory";
import { formatCurrency } from "../../utils/formatters";

interface InvoiceLineItem {
  item_id: string;
  item_name: string;
  quantity: number;
  unit_price: number;
  discount: number;
  total: number;
}

interface SalesInvoiceFormProps {
  onSubmit: (values: any) => Promise<void>;
  loading?: boolean;
}

export function SalesInvoiceForm({ onSubmit, loading }: SalesInvoiceFormProps) {
  const [items, setItems] = useState<InvoiceLineItem[]>([]);

  const { data: customersData } = useQuery({
    queryKey: ["customers-select"],
    queryFn: () => customersApi.list({ per_page: 200 }),
  });

  const { data: inventoryData } = useQuery({
    queryKey: ["inventory-select"],
    queryFn: () => inventoryApi.list({ per_page: 500 }),
  });

  const form = useForm({
    initialValues: {
      customer_id: "",
      branch_id: "",
      due_date: null as Date | null,
      notes: "",
      discount_amount: 0,
      tax_rate: 0,
    },
    validate: {
      customer_id: (v) => (!v ? "Customer required" : null),
    },
  });

  const customerOptions =
    customersData?.data?.map((c) => ({
      value: c.id,
      label: c.name,
    })) || [];

  const inventoryOptions =
    inventoryData?.data?.map((i) => ({
      value: i.id,
      label: `${i.sku} - ${i.name}`,
      price: i.unit_price,
    })) || [];

  const addItem = () => {
    setItems((prev) => [
      ...prev,
      {
        item_id: "",
        item_name: "",
        quantity: 1,
        unit_price: 0,
        discount: 0,
        total: 0,
      },
    ]);
  };

  const removeItem = (index: number) => {
    setItems((prev) => prev.filter((_, i) => i !== index));
  };

  const updateItem = (index: number, field: string, value: any) => {
    setItems((prev) =>
      prev.map((item, i) => {
        if (i !== index) return item;
        const updated = { ...item, [field]: value };
        if (field === "item_id") {
          const found = inventoryOptions.find((o) => o.value === value);
          updated.item_name = found?.label || "";
          updated.unit_price = found?.price || 0;
        }
        updated.total =
          updated.quantity * updated.unit_price * (1 - updated.discount / 100);
        return updated;
      }),
    );
  };

  const subtotal = items.reduce((sum, i) => sum + i.total, 0);
  const tax = subtotal * (form.values.tax_rate / 100);
  const grandTotal = subtotal + tax - form.values.discount_amount;

  const handleSubmit = async (values: typeof form.values) => {
    if (items.length === 0) return;
    await onSubmit({
      ...values,
      items: items.map((i) => ({
        item_id: i.item_id,
        quantity: i.quantity,
        unit_price: i.unit_price,
        discount: i.discount,
      })),
    });
  };

  return (
    <form onSubmit={form.onSubmit(handleSubmit)}>
      <Stack>
        <Group grow>
          <Select
            label="Customer"
            placeholder="Select customer"
            data={customerOptions}
            required
            searchable
            {...form.getInputProps("customer_id")}
          />
          <DatePickerInput
            label="Due Date"
            placeholder="Select due date"
            valueFormat="MMM DD, YYYY"
            {...form.getInputProps("due_date")}
          />
        </Group>

        <Divider label="Line Items" />

        {items.map((item, index) => (
          <Paper key={index} withBorder p="sm" radius="md">
            <Group align="flex-end">
              <Select
                label="Item"
                placeholder="Select item"
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
                label="Price"
                prefix="$"
                decimalScale={2}
                style={{ width: 110 }}
                value={item.unit_price}
                onChange={(v) => updateItem(index, "unit_price", Number(v))}
              />
              <NumberInput
                label="Disc %"
                suffix="%"
                min={0}
                max={100}
                style={{ width: 90 }}
                value={item.discount}
                onChange={(v) => updateItem(index, "discount", Number(v))}
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

        <Group grow>
          <NumberInput
            label="Tax %"
            suffix="%"
            min={0}
            max={100}
            {...form.getInputProps("tax_rate")}
          />
          <NumberInput
            label="Discount ($)"
            prefix="$"
            min={0}
            {...form.getInputProps("discount_amount")}
          />
        </Group>

        <Paper withBorder p="md" radius="md">
          <Stack gap="xs">
            <Group justify="space-between">
              <Text c="dimmed">Subtotal</Text>
              <Text>{formatCurrency(subtotal)}</Text>
            </Group>
            <Group justify="space-between">
              <Text c="dimmed">Tax ({form.values.tax_rate}%)</Text>
              <Text>{formatCurrency(tax)}</Text>
            </Group>
            <Group justify="space-between">
              <Text c="dimmed">Discount</Text>
              <Text c="red">
                -{formatCurrency(form.values.discount_amount)}
              </Text>
            </Group>
            <Divider />
            <Group justify="space-between">
              <Text fw={700}>Total</Text>
              <Text fw={700} size="lg">
                {formatCurrency(grandTotal)}
              </Text>
            </Group>
          </Stack>
        </Paper>

        <Textarea label="Notes" rows={3} {...form.getInputProps("notes")} />

        <Button
          type="submit"
          loading={loading}
          disabled={items.length === 0}
          size="md"
        >
          Create Invoice
        </Button>
      </Stack>
    </form>
  );
}
