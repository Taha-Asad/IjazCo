import {
  TextInput,
  Textarea,
  NumberInput,
  Select,
  Button,
  Stack,
  SimpleGrid,
  Switch,
} from "@mantine/core";
import { useForm } from "@mantine/form";
import { z } from "zod";
import { zod4Resolver } from "mantine-form-zod-resolver";
import { useQuery } from "@tanstack/react-query";
import { categoriesApi } from "../../api/categories";

const schema = z.object({
  name: z.string().min(2, "Name required"),
  sku: z.string().min(1, "SKU required"),
  unit_price: z.number().min(0, "Price must be positive"),
  cost_price: z.number().min(0),
  unit: z.string().min(1, "Unit required"),
  min_stock_level: z.number().min(0),
});

interface InventoryItemFormProps {
  initialValues?: any;
  onSubmit: (values: any) => Promise<void>;
  loading?: boolean;
}

export function InventoryItemForm({
  initialValues,
  onSubmit,
  loading,
}: InventoryItemFormProps) {
  const { data: categoriesData } = useQuery({
    queryKey: ["categories-list"],
    queryFn: () => categoriesApi.list({ per_page: 200 }),
  });

  const categoryOptions =
    categoriesData?.data?.map((c) => ({
      value: c.id,
      label: c.name,
    })) || [];

  const form = useForm({
    validate: zod4Resolver(schema),
    initialValues: {
      name: initialValues?.name || "",
      sku: initialValues?.sku || "",
      description: initialValues?.description || "",
      category_id: initialValues?.category_id || null,
      unit_price: initialValues?.unit_price || 0,
      cost_price: initialValues?.cost_price || 0,
      unit: initialValues?.unit || "pcs",
      min_stock_level: initialValues?.min_stock_level || 0,
      max_stock_level: initialValues?.max_stock_level || null,
      serial_number: initialValues?.serial_number || "",
      is_active: initialValues?.is_active ?? true,
    },
  });

  const UNIT_OPTIONS = [
    { value: "pcs", label: "Pieces" },
    { value: "kg", label: "Kilograms" },
    { value: "g", label: "Grams" },
    { value: "l", label: "Liters" },
    { value: "ml", label: "Milliliters" },
    { value: "box", label: "Box" },
    { value: "set", label: "Set" },
  ];

  return (
    <form onSubmit={form.onSubmit(onSubmit)}>
      <Stack>
        <SimpleGrid cols={2}>
          <TextInput
            label="Item Name"
            placeholder="Microscope XL-200"
            required
            {...form.getInputProps("name")}
          />
          <TextInput
            label="SKU"
            placeholder="MIC-XL-200"
            required
            {...form.getInputProps("sku")}
          />
        </SimpleGrid>
        <TextInput
          label="Serial Number"
          placeholder="SN-00000"
          {...form.getInputProps("serial_number")}
        />
        <Select
          label="Category"
          placeholder="Select category"
          data={categoryOptions}
          clearable
          {...form.getInputProps("category_id")}
        />
        <Textarea
          label="Description"
          rows={2}
          {...form.getInputProps("description")}
        />
        <SimpleGrid cols={2}>
          <NumberInput
            label="Selling Price"
            prefix="$"
            thousandSeparator=","
            decimalScale={2}
            min={0}
            required
            {...form.getInputProps("unit_price")}
          />
          <NumberInput
            label="Cost Price"
            prefix="$"
            thousandSeparator=","
            decimalScale={2}
            min={0}
            {...form.getInputProps("cost_price")}
          />
        </SimpleGrid>
        <SimpleGrid cols={3}>
          <Select
            label="Unit"
            data={UNIT_OPTIONS}
            required
            {...form.getInputProps("unit")}
          />
          <NumberInput
            label="Min Stock Level"
            min={0}
            {...form.getInputProps("min_stock_level")}
          />
          <NumberInput
            label="Max Stock Level"
            min={0}
            __clearable
            {...form.getInputProps("max_stock_level")}
          />
        </SimpleGrid>
        {initialValues && (
          <Switch
            label="Active"
            {...form.getInputProps("is_active", { type: "checkbox" })}
          />
        )}
        <Button type="submit" loading={loading}>
          {initialValues ? "Save Changes" : "Create Item"}
        </Button>
      </Stack>
    </form>
  );
}
