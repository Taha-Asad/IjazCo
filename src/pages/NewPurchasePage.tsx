import { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { createPurchaseOrder } from '../services/purchases';
import { getSuppliers, Supplier } from '../services/suppliers';
import { getInventoryItems, InventoryItem } from '../services/inventory';
import './FormPage.css';

export default function NewPurchasePage() {
  const navigate = useNavigate();
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');
  
  // Data sources
  const [suppliers, setSuppliers] = useState<Supplier[]>([]);
  const [inventory, setInventory] = useState<InventoryItem[]>([]);
  
  const [form, setForm] = useState({
    supplier_id: '',
    branch_id: '00000000-0000-0000-0000-000000000000', // Default mock branch
    discount_amount: 0,
    shipping_amount: 0,
    notes: '',
  });

  const [items, setItems] = useState<Array<{ item_id: string; quantity_ordered: number; unit_cost: number; tax_percentage: number }>>([
    { item_id: '', quantity_ordered: 1, unit_cost: 0, tax_percentage: 0 }
  ]);

  useEffect(() => {
    // Load suppliers and inventory for dropdowns
    getSuppliers({ limit: 100 }).then(res => setSuppliers(res.suppliers)).catch(console.error);
    getInventoryItems({ limit: 100 }).then(res => setInventory(res.items)).catch(console.error);
  }, []);

  const handleFormChange = (e: React.ChangeEvent<HTMLSelectElement | HTMLInputElement | HTMLTextAreaElement>) => {
    setForm(prev => ({ ...prev, [e.target.name]: e.target.value }));
  };

  const handleItemChange = (index: number, field: string, value: string | number) => {
    const newItems = [...items];
    
    // Auto-fill cost when item changes
    if (field === 'item_id') {
      const selectedItem = inventory.find(i => i.id === value);
      if (selectedItem) {
        newItems[index].unit_cost = selectedItem.cost_price || 0;
      }
    }
    
    newItems[index] = { ...newItems[index], [field]: value };
    setItems(newItems);
  };

  const addItemRow = () => setItems([...items, { item_id: '', quantity_ordered: 1, unit_cost: 0, tax_percentage: 0 }]);
  const removeItemRow = (index: number) => setItems(items.filter((_, i) => i !== index));

  const calculateTotal = () => {
    let subtotal = 0;
    let taxTotal = 0;
    
    items.forEach(item => {
      const lineSubtotal = item.quantity_ordered * item.unit_cost;
      const tax = lineSubtotal * (item.tax_percentage / 100);
      subtotal += lineSubtotal;
      taxTotal += tax;
    });
    
    return (subtotal + taxTotal + Number(form.shipping_amount) - Number(form.discount_amount)).toFixed(2);
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError('');
    
    if (!form.supplier_id) {
      setError('Please select a supplier.');
      return;
    }

    const validItems = items.filter(i => i.item_id && i.quantity_ordered > 0);
    if (validItems.length === 0) {
      setError('Please add at least one valid item to the purchase order.');
      return;
    }
    
    try {
      setLoading(true);
      await createPurchaseOrder({
        supplier_id: form.supplier_id,
        branch_id: form.branch_id,
        discount_amount: Number(form.discount_amount),
        shipping_amount: Number(form.shipping_amount),
        notes: form.notes,
        items: validItems.map(item => ({
          ...item,
          tax_percentage: Number(item.tax_percentage)
        })),
      });
      navigate('/purchases');
    } catch (err: any) {
      setError(err.message || 'Failed to create purchase order.');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="form-root">
      <div className="form-header">
        <div className="form-header__text">
          <h1 className="form-title">Create Purchase Order</h1>
          <p className="form-subtitle">Record a new procurement order from a supplier.</p>
        </div>
        <button className="form-btn form-btn--ghost" onClick={() => navigate('/purchases')}>
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
        
        {/* PO Metadata */}
        <div className="form-grid" style={{ paddingBottom: '24px', borderBottom: '1px solid var(--c-border)' }}>
          <div className="form-group form-group--full">
            <label className="form-label">Supplier *</label>
            <select className="form-input" name="supplier_id" value={form.supplier_id} onChange={handleFormChange} required>
              <option value="">-- Select Supplier --</option>
              {suppliers.map(s => (
                <option key={s.id} value={s.id}>{s.name} ({s.contact_person || 'No Contact'})</option>
              ))}
            </select>
          </div>
          
          <div className="form-group">
            <label className="form-label">Branch ID (UUID)</label>
            <input className="form-input" name="branch_id" value={form.branch_id} onChange={handleFormChange} required />
          </div>
          
          <div className="form-group form-group--full">
            <label className="form-label">Notes</label>
            <textarea 
              className="form-input" 
              name="notes" 
              value={form.notes} 
              onChange={handleFormChange} 
              style={{ minHeight: '60px', resize: 'vertical' }} 
            />
          </div>
        </div>

        {/* PO Items */}
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
                <div className="form-group" style={{ flex: 3, margin: 0 }}>
                  <label className="form-label" style={{ fontSize: '0.75rem' }}>Product/Item</label>
                  <select className="form-input" value={item.item_id} onChange={(e) => handleItemChange(index, 'item_id', e.target.value)} required>
                    <option value="">-- Select --</option>
                    {inventory.map(inv => (
                      <option key={inv.id} value={inv.id}>{inv.name} (Cost: ${inv.cost_price})</option>
                    ))}
                  </select>
                </div>
                <div className="form-group" style={{ flex: 1, margin: 0 }}>
                  <label className="form-label" style={{ fontSize: '0.75rem' }}>Qty</label>
                  <input className="form-input" type="number" min="1" value={item.quantity_ordered} onChange={(e) => handleItemChange(index, 'quantity_ordered', Number(e.target.value))} required />
                </div>
                <div className="form-group" style={{ flex: 1, margin: 0 }}>
                  <label className="form-label" style={{ fontSize: '0.75rem' }}>Unit Cost ($)</label>
                  <input className="form-input" type="number" step="0.01" value={item.unit_cost} onChange={(e) => handleItemChange(index, 'unit_cost', Number(e.target.value))} required />
                </div>
                <div className="form-group" style={{ flex: 1, margin: 0 }}>
                  <label className="form-label" style={{ fontSize: '0.75rem' }}>Tax (%)</label>
                  <input className="form-input" type="number" step="0.1" value={item.tax_percentage} onChange={(e) => handleItemChange(index, 'tax_percentage', Number(e.target.value))} />
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
            <label className="form-label">Shipping Amount ($)</label>
            <input className="form-input" type="number" step="0.01" name="shipping_amount" value={form.shipping_amount} onChange={handleFormChange} />
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
          <button type="button" className="form-btn form-btn--ghost" onClick={() => navigate('/purchases')} disabled={loading}>
            Cancel
          </button>
          <button type="submit" className="form-btn form-btn--primary" disabled={loading}>
            {loading ? 'Creating...' : 'Create Purchase Order'}
          </button>
        </div>
      </form>
    </div>
  );
}
