import { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { createSalesInvoice } from '../services/sales';
import { getCustomers, Customer } from '../services/customers';
import { getInventoryItems, InventoryItem } from '../services/inventory';
import './FormPage.css';

export default function NewSalePage() {
  const navigate = useNavigate();
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');
  
  // Data sources
  const [customers, setCustomers] = useState<Customer[]>([]);
  const [inventory, setInventory] = useState<InventoryItem[]>([]);
  
  const [form, setForm] = useState({
    customer_id: '',
    branch_id: '00000000-0000-0000-0000-000000000000', // Default mock branch
    tax_rate: 0,
    discount_amount: 0,
  });

  const [items, setItems] = useState<Array<{ item_id: string; quantity: number; unit_price: number }>>([
    { item_id: '', quantity: 1, unit_price: 0 }
  ]);

  useEffect(() => {
    // Load customers and inventory for dropdowns
    getCustomers({ limit: 100 }).then(res => setCustomers(res.customers)).catch(console.error);
    getInventoryItems({ limit: 100 }).then(res => setInventory(res.items)).catch(console.error);
  }, []);

  const handleFormChange = (e: React.ChangeEvent<HTMLSelectElement | HTMLInputElement>) => {
    setForm(prev => ({ ...prev, [e.target.name]: e.target.value }));
  };

  const handleItemChange = (index: number, field: string, value: string | number) => {
    const newItems = [...items];
    
    // Auto-fill price when item changes
    if (field === 'item_id') {
      const selectedItem = inventory.find(i => i.id === value);
      if (selectedItem) {
        newItems[index].unit_price = selectedItem.selling_price;
      }
    }
    
    newItems[index] = { ...newItems[index], [field]: value };
    setItems(newItems);
  };

  const addItemRow = () => setItems([...items, { item_id: '', quantity: 1, unit_price: 0 }]);
  const removeItemRow = (index: number) => setItems(items.filter((_, i) => i !== index));

  const calculateTotal = () => {
    const subtotal = items.reduce((sum, item) => sum + (item.quantity * item.unit_price), 0);
    const tax = subtotal * (Number(form.tax_rate) / 100);
    return (subtotal + tax - Number(form.discount_amount)).toFixed(2);
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError('');
    
    if (!form.customer_id) {
      setError('Please select a customer.');
      return;
    }

    const validItems = items.filter(i => i.item_id && i.quantity > 0);
    if (validItems.length === 0) {
      setError('Please add at least one valid item to the invoice.');
      return;
    }
    
    try {
      setLoading(true);
      await createSalesInvoice({
        customer_id: form.customer_id,
        branch_id: form.branch_id,
        tax_rate: Number(form.tax_rate),
        discount_amount: Number(form.discount_amount),
        items: validItems,
      });
      navigate('/sales');
    } catch (err: any) {
      setError(err.message || 'Failed to create invoice.');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="form-root">
      <div className="form-header">
        <div className="form-header__text">
          <h1 className="form-title">Create Sales Invoice</h1>
          <p className="form-subtitle">Record a new transaction and generate an invoice.</p>
        </div>
        <button className="form-btn form-btn--ghost" onClick={() => navigate('/sales')}>
          Cancel
        </button>
      </div>

      {error && (
        <div className="form-error">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><circle cx="12" cy="12" r="10"/><line x1="12" y1="8" x2="12" y2="12"/></svg>
          {error}
        </div>
      )}

      <form className="form-card" onSubmit={handleSubmit} style={{ display: 'flex', flexDirection: 'column', gap: '24px' }}>
        
        {/* Invoice Metadata */}
        <div className="form-grid" style={{ paddingBottom: '24px', borderBottom: '1px solid var(--c-border)' }}>
          <div className="form-group form-group--full">
            <label className="form-label">Customer *</label>
            <select className="form-input" name="customer_id" value={form.customer_id} onChange={handleFormChange} required>
              <option value="">-- Select Customer --</option>
              {customers.map(c => (
                <option key={c.id} value={c.id}>{c.name} ({c.email})</option>
              ))}
            </select>
          </div>
          
          <div className="form-group">
            <label className="form-label">Branch ID (UUID)</label>
            <input className="form-input" name="branch_id" value={form.branch_id} onChange={handleFormChange} required />
          </div>
        </div>

        {/* Invoice Items */}
        <div style={{ padding: '0 24px' }}>
          <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '16px' }}>
            <h3 style={{ margin: 0, fontSize: '1rem', fontWeight: 600 }}>Line Items</h3>
            <button type="button" onClick={addItemRow} className="form-btn form-btn--ghost" style={{ height: '32px', fontSize: '0.8rem' }}>
              + Add Item
            </button>
          </div>
          
          <div style={{ display: 'flex', flexDirection: 'column', gap: '12px' }}>
            {items.map((item, index) => (
              <div key={index} style={{ display: 'flex', gap: '12px', alignItems: 'flex-end' }}>
                <div className="form-group" style={{ flex: 2, margin: 0 }}>
                  <label className="form-label" style={{ fontSize: '0.75rem' }}>Product/Item</label>
                  <select className="form-input" value={item.item_id} onChange={(e) => handleItemChange(index, 'item_id', e.target.value)} required>
                    <option value="">-- Select --</option>
                    {inventory.map(inv => (
                      <option key={inv.id} value={inv.id}>{inv.name} (${inv.selling_price})</option>
                    ))}
                  </select>
                </div>
                <div className="form-group" style={{ flex: 1, margin: 0 }}>
                  <label className="form-label" style={{ fontSize: '0.75rem' }}>Qty</label>
                  <input className="form-input" type="number" min="1" value={item.quantity} onChange={(e) => handleItemChange(index, 'quantity', Number(e.target.value))} required />
                </div>
                <div className="form-group" style={{ flex: 1, margin: 0 }}>
                  <label className="form-label" style={{ fontSize: '0.75rem' }}>Unit Price ($)</label>
                  <input className="form-input" type="number" step="0.01" value={item.unit_price} onChange={(e) => handleItemChange(index, 'unit_price', Number(e.target.value))} required />
                </div>
                <button type="button" onClick={() => removeItemRow(index)} className="form-btn form-btn--ghost" style={{ width: '42px', padding: 0, color: '#dc2626' }}>
                  <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><polyline points="3 6 5 6 21 6"/><path d="M19 6l-1 14H6L5 6"/><path d="M10 11v6"/><path d="M14 11v6"/></svg>
                </button>
              </div>
            ))}
          </div>
        </div>

        {/* Totals & Discounts */}
        <div className="form-grid" style={{ paddingTop: '24px', borderTop: '1px solid var(--c-border)', background: 'var(--c-bg)', paddingBottom: '24px' }}>
          <div className="form-group">
            <label className="form-label">Tax Rate (%)</label>
            <input className="form-input" type="number" step="0.1" name="tax_rate" value={form.tax_rate} onChange={handleFormChange} />
          </div>
          <div className="form-group">
            <label className="form-label">Discount Amount ($)</label>
            <input className="form-input" type="number" step="0.01" name="discount_amount" value={form.discount_amount} onChange={handleFormChange} />
          </div>
          <div className="form-group form-group--full" style={{ textAlign: 'right' }}>
            <div style={{ fontSize: '0.875rem', color: 'var(--c-slate-500)', marginBottom: '4px' }}>Total Amount</div>
            <div style={{ fontSize: '1.75rem', fontWeight: 700, color: 'var(--c-slate-900)' }}>
              ${calculateTotal()}
            </div>
          </div>
        </div>

        <div className="form-footer">
          <button type="button" className="form-btn form-btn--ghost" onClick={() => navigate('/sales')} disabled={loading}>
            Cancel
          </button>
          <button type="submit" className="form-btn form-btn--primary" disabled={loading}>
            {loading ? 'Creating...' : 'Create Invoice'}
          </button>
        </div>
      </form>
    </div>
  );
}
