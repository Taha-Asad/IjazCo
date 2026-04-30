import { useState, useEffect } from 'react';
import { useNavigate, useParams } from 'react-router-dom';
import { getInventoryItems, updateInventoryItem, getCategories, Category } from '../services/inventory';
import './FormPage.css';

export default function EditInventoryPage() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState('');
  const [categories, setCategories] = useState<Category[]>([]);
  
  const [form, setForm] = useState({
    sku: '',
    name: '',
    description: '',
    category_id: '',
    cost_price: '',
    selling_price: '',
    reorder_level: '',
  });

  useEffect(() => {
    if (id) loadData();
  }, [id]);

  const loadData = async () => {
    try {
      setLoading(true);
      setError('');
      
      const [cats, itemsRes] = await Promise.all([
        getCategories().catch(() => []),
        getInventoryItems({ search: id, limit: 1 }).catch(() => ({ items: [], total: 0 }))
      ]);
      
      setCategories(cats);
      
      // Because we don't have a getInventoryItem(id) yet, we find it from the list
      // Or in a real app, you'd use a dedicated GET /inventory/:id endpoint
      // We will assume `getInventoryItems` search by ID works or we'll fetch the whole page
      let item = itemsRes.items.find(i => i.id === id);
      if (!item) {
        // Fallback: fetch without search to find it
        const all = await getInventoryItems({ limit: 1000 });
        item = all.items.find(i => i.id === id);
      }
      
      if (!item) throw new Error('Item not found');

      setForm({
        sku: item.sku || '',
        name: item.name || '',
        description: item.description || '',
        category_id: item.category_id || '',
        cost_price: String(item.cost_price || ''),
        selling_price: String(item.selling_price || ''),
        reorder_level: String(item.reorder_level || ''),
      });
      
    } catch (err: any) {
      setError(err.message || 'Failed to load item data.');
    } finally {
      setLoading(false);
    }
  };

  const handleChange = (e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement | HTMLTextAreaElement>) => {
    setForm(prev => ({ ...prev, [e.target.name]: e.target.value }));
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError('');
    
    if (!form.sku || !form.name || !form.cost_price || !form.selling_price || !id) {
      setError('Please fill out all required fields.');
      return;
    }
    
    try {
      setSaving(true);
      await updateInventoryItem(id, {
        ...form,
        cost_price: Number(form.cost_price),
        selling_price: Number(form.selling_price),
        reorder_level: Number(form.reorder_level) || 0,
      });
      navigate('/inventory');
    } catch (err: any) {
      setError(err.message || 'Failed to update item.');
    } finally {
      setSaving(false);
    }
  };

  if (loading) {
    return (
      <div className="form-root">
        <div className="form-loading">Loading item data...</div>
      </div>
    );
  }

  return (
    <div className="form-root">
      <div className="form-header">
        <div className="form-header__text">
          <h1 className="form-title">Edit Item</h1>
          <p className="form-subtitle">Update inventory product details.</p>
        </div>
        <button className="form-btn form-btn--ghost" onClick={() => navigate('/inventory')}>
          Cancel
        </button>
      </div>

      {error && (
        <div className="form-error">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><circle cx="12" cy="12" r="10"/><line x1="12" y1="8" x2="12" y2="12"/></svg>
          {error}
        </div>
      )}

      <form className="form-card" onSubmit={handleSubmit}>
        <div className="form-grid">
          <div className="form-group">
            <label className="form-label">SKU *</label>
            <input className="form-input" name="sku" value={form.sku} onChange={handleChange} required />
          </div>
          <div className="form-group">
            <label className="form-label">Product Name *</label>
            <input className="form-input" name="name" value={form.name} onChange={handleChange} required />
          </div>
          
          <div className="form-group form-group--full">
            <label className="form-label">Description</label>
            <textarea className="form-input" name="description" value={form.description} onChange={handleChange} />
          </div>

          <div className="form-group">
            <label className="form-label">Category</label>
            <select className="form-input" name="category_id" value={form.category_id} onChange={handleChange}>
              <option value="">No Category</option>
              {categories.map(c => (
                <option key={c.id} value={c.id}>{c.name}</option>
              ))}
            </select>
          </div>
          <div className="form-group">
            <label className="form-label">Reorder Level</label>
            <input className="form-input" type="number" name="reorder_level" value={form.reorder_level} onChange={handleChange} />
          </div>

          <div className="form-group">
            <label className="form-label">Cost Price *</label>
            <input className="form-input" type="number" step="0.01" name="cost_price" value={form.cost_price} onChange={handleChange} required />
          </div>
          <div className="form-group">
            <label className="form-label">Selling Price *</label>
            <input className="form-input" type="number" step="0.01" name="selling_price" value={form.selling_price} onChange={handleChange} required />
          </div>
        </div>

        <div className="form-footer">
          <button type="button" className="form-btn form-btn--ghost" onClick={() => navigate('/inventory')} disabled={saving}>
            Cancel
          </button>
          <button type="submit" className="form-btn form-btn--primary" disabled={saving}>
            {saving ? 'Updating...' : 'Update Item'}
          </button>
        </div>
      </form>
    </div>
  );
}
