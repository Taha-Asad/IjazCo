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
  unit_of_measure: z.string().min(1, "Unit required"),
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
      unit_of_measure: initialValues?.unit_of_measure || initialValues?.unit || "pcs",
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

  const handleSubmit = (values: any) => {
    // Clean up data for backend
    const submitData = { ...values };
    // Map unit_price to selling_price for backend
    if (submitData.unit_price !== undefined) {
      submitData.selling_price = submitData.unit_price;
      delete submitData.unit_price;
    }
    // Remove any 'unit' field if present (we use unit_of_measure now)
    if (submitData.unit) {
      delete submitData.unit;
    }
    // Add required fields with defaults
    submitData.is_serialized = false;
    submitData.is_batch_tracked = false;
    submitData.reorder_level = 0;
    submitData.reorder_quantity = 0;
    submitData.lead_time_days = 0;
    if (!submitData.tax_rate) {
      submitData.tax_rate = 0;
    }
    return onSubmit(submitData);
  };

  return (
    <form onSubmit={form.onSubmit(handleSubmit)}>
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
          value={form.values.category_id || ""}
          onChange={(val) => form.setFieldValue("category_id", val || null)}
          error={form.errors.category_id}
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
            value={form.values.unit_of_measure || ""}
            onChange={(val) => form.setFieldValue("unit_of_measure", val)}
            error={form.errors.unit_of_measure}
          />
          <NumberInput
            label="Min Stock Level"
            min={0}
            {...form.getInputProps("min_stock_level")}
          />
          <NumberInput
            label="Max Stock Level"
            min={0}
            value={form.values.max_stock_level}
            onChange={(val) => form.setFieldValue("max_stock_level", val)}
            error={form.errors.max_stock_level}
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
