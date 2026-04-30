import dayjs from "dayjs";

export function formatCurrency(
  amount: number,
  currency = "USD",
  locale = "en-US",
): string {
  return new Intl.NumberFormat(locale, {
    style: "currency",
    currency,
    minimumFractionDigits: 2,
  }).format(amount);
}

export function formatDate(
  date: string | Date,
  format = "MMM DD, YYYY",
): string {
  return dayjs(date).format(format);
}

export function formatDateTime(
  date: string | Date,
  format = "MMM DD, YYYY HH:mm",
): string {
  return dayjs(date).format(format);
}

export function formatNumber(n: number): string {
  return new Intl.NumberFormat().format(n);
}

export function formatPercentage(n: number, decimals = 1): string {
  return `${n.toFixed(decimals)}%`;
}

export function truncateText(text: string, maxLength = 50): string {
  return text.length > maxLength ? `${text.substring(0, maxLength)}...` : text;
}

export function formatFileSize(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
}
