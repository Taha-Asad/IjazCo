import { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { createInventoryItem, getCategories, Category } from '../services/inventory';
import './FormPage.css';

export default function NewItemPage() {
  const navigate = useNavigate();
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');
  const [categories, setCategories] = useState<Category[]>([]);
  
  const [form, setForm] = useState({
    sku: '',
    name: '',
    description: '',
    category_id: '',
    cost_price: '',
    selling_price: '',
    quantity_on_hand: '',
    reorder_level: '',
  });

  useEffect(() => {
    loadCategories();
  }, []);

  const loadCategories = async () => {
    try {
      const cats = await getCategories();
      setCategories(cats);
    } catch (err) {
      console.error('Failed to load categories', err);
    }
  };

  const handleChange = (e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement | HTMLTextAreaElement>) => {
    setForm(prev => ({ ...prev, [e.target.name]: e.target.value }));
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError('');
    
    if (!form.sku || !form.name || !form.cost_price || !form.selling_price) {
      setError('Please fill out all required fields.');
      return;
    }
    
    try {
      setLoading(true);
      await createInventoryItem({
        ...form,
        cost_price: Number(form.cost_price),
        selling_price: Number(form.selling_price),
        reorder_level: Number(form.reorder_level) || 0,
      });
      navigate('/inventory');
    } catch (err: any) {
      setError(err.message || 'Failed to create item. SKU might already exist.');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="form-root">
      <div className="form-header">
        <div className="form-header__text">
          <h1 className="form-title">Add Item</h1>
          <p className="form-subtitle">Add a new inventory product.</p>
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
            <input className="form-input" name="sku" value={form.sku} onChange={handleChange} required placeholder="e.g. PRD-001" />
          </div>
          <div className="form-group">
            <label className="form-label">Product Name *</label>
            <input className="form-input" name="name" value={form.name} onChange={handleChange} required />
          </div>
          
          <div className="form-group form-group--full">
            <label className="form-label">Description</label>
            <textarea className="form-input" name="description" value={form.description} onChange={handleChange} placeholder="Optional item details..." />
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
            <label className="form-label">Cost Price *</label>
            <input className="form-input" type="number" step="0.01" name="cost_price" value={form.cost_price} onChange={handleChange} required />
          </div>
          
          <div className="form-group">
            <label className="form-label">Selling Price *</label>
            <input className="form-input" type="number" step="0.01" name="selling_price" value={form.selling_price} onChange={handleChange} required />
          </div>
          <div className="form-group">
            <label className="form-label">Initial Quantity</label>
            <input className="form-input" type="number" name="quantity_on_hand" value={form.quantity_on_hand} onChange={handleChange} placeholder="0" />
          </div>
          
          <div className="form-group">
            <label className="form-label">Reorder Level</label>
            <input className="form-input" type="number" name="reorder_level" value={form.reorder_level} onChange={handleChange} placeholder="10" />
          </div>
        </div>

        <div className="form-footer">
          <button type="button" className="form-btn form-btn--ghost" onClick={() => navigate('/inventory')} disabled={loading}>
            Cancel
          </button>
          <button type="submit" className="form-btn form-btn--primary" disabled={loading}>
            {loading ? 'Saving...' : 'Save Item'}
          </button>
        </div>
      </form>
    </div>
  );
}
