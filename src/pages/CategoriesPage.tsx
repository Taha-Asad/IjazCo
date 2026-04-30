import React, { useState, useEffect } from 'react';
import { getCategories, createCategory, updateCategory, deleteCategory, Category } from '../services/categories';
import { useToast } from '../contexts/ToastContext';
import './CategoriesPage.css';

export default function CategoriesPage() {
  const [categories, setCategories] = useState<Category[]>([]);
  const [loading, setLoading] = useState(true);
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [editingCategory, setEditingCategory] = useState<Category | null>(null);
  
  const [formData, setFormData] = useState({ name: '', description: '' });
  const [isSubmitting, setIsSubmitting] = useState(false);

  const { success, error } = useToast();

  const loadCategories = async () => {
    try {
      setLoading(true);
      const data = await getCategories({ limit: 100 });
      setCategories(Array.isArray(data.categories) ? data.categories : []);
    } catch (err: any) {
      error(err.message || 'Failed to load categories');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadCategories();
  }, []);

  const openNewModal = () => {
    setEditingCategory(null);
    setFormData({ name: '', description: '' });
    setIsModalOpen(true);
  };

  const openEditModal = (cat: Category) => {
    setEditingCategory(cat);
    setFormData({ name: cat.name, description: cat.description || '' });
    setIsModalOpen(true);
  };

  const closeModal = () => {
    setIsModalOpen(false);
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!formData.name.trim()) return error('Category name is required');
    
    try {
      setIsSubmitting(true);
      if (editingCategory) {
        await updateCategory(editingCategory.id, formData);
        success('Category updated successfully');
      } else {
        await createCategory(formData);
        success('Category created successfully');
      }
      closeModal();
      loadCategories();
    } catch (err: any) {
      error(err.message || 'Failed to save category');
    } finally {
      setIsSubmitting(false);
    }
  };

  const handleDelete = async (id: string, name: string) => {
    if (!window.confirm(`Are you sure you want to delete category "${name}"?`)) return;
    
    try {
      await deleteCategory(id);
      success('Category deleted successfully');
      loadCategories();
    } catch (err: any) {
      error(err.message || 'Failed to delete category');
    }
  };

  return (
    <div className="categories-page animate-fade-in">
      <div className="categories-header">
        <div>
          <h1 className="categories-header__title">Categories</h1>
          <p className="categories-header__subtitle">Manage your product categories and tax classifications.</p>
        </div>
        <button className="btn-primary" onClick={openNewModal}>
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
            <line x1="12" y1="5" x2="12" y2="19"></line>
            <line x1="5" y1="12" x2="19" y2="12"></line>
          </svg>
          Add Category
        </button>
      </div>

      <div className="categories-content">
        {loading ? (
          <div className="loading-state">Loading categories...</div>
        ) : categories.length === 0 ? (
          <div className="empty-state">
            <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1" strokeLinecap="round" strokeLinejoin="round">
              <path d="M4 9h16M4 15h16M10 3L8 21M16 3l-2 18"/>
            </svg>
            <h3>No categories found</h3>
            <p>Get started by creating your first product category.</p>
          </div>
        ) : (
          <div className="categories-table-container">
            <table className="categories-table">
              <thead>
                <tr>
                  <th>Name</th>
                  <th>Description</th>
                  <th>Slug</th>
                  <th style={{ textAlign: 'right' }}>Actions</th>
                </tr>
              </thead>
              <tbody>
                {categories.map((cat) => (
                  <tr key={cat.id}>
                    <td>
                      <div className="category-name">{cat.name}</div>
                    </td>
                    <td>{cat.description || '-'}</td>
                    <td><code style={{ fontSize: '0.8125rem', background: 'var(--c-slate-100)', padding: '2px 6px', borderRadius: '4px' }}>{cat.slug}</code></td>
                    <td>
                      <div className="actions-cell">
                        <button className="btn-icon" onClick={() => openEditModal(cat)} title="Edit Category">
                          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"></path><path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"></path></svg>
                        </button>
                        <button className="btn-icon btn-icon--danger" onClick={() => handleDelete(cat.id, cat.name)} title="Delete Category">
                          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><polyline points="3 6 5 6 21 6"></polyline><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path><line x1="10" y1="11" x2="10" y2="17"></line><line x1="14" y1="11" x2="14" y2="17"></line></svg>
                        </button>
                      </div>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}
      </div>

      {/* Modal */}
      {isModalOpen && (
        <div className="modal-overlay" onClick={closeModal}>
          <div className="modal-content" onClick={e => e.stopPropagation()}>
            <form onSubmit={handleSubmit}>
              <div className="modal-header">
                <h2>{editingCategory ? 'Edit Category' : 'New Category'}</h2>
                <button type="button" className="modal-close" onClick={closeModal}>
                  <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><line x1="18" y1="6" x2="6" y2="18"></line><line x1="6" y1="6" x2="18" y2="18"></line></svg>
                </button>
              </div>
              <div className="modal-body">
                <div className="form-group">
                  <label htmlFor="name">Category Name <span style={{color: '#ef4444'}}>*</span></label>
                  <input
                    id="name"
                    type="text"
                    className="form-control"
                    value={formData.name}
                    onChange={e => setFormData({ ...formData, name: e.target.value })}
                    placeholder="e.g. Antibiotics"
                    autoFocus
                    required
                  />
                </div>
                <div className="form-group">
                  <label htmlFor="description">Description</label>
                  <textarea
                    id="description"
                    className="form-control"
                    value={formData.description}
                    onChange={e => setFormData({ ...formData, description: e.target.value })}
                    placeholder="Brief description of this category..."
                    rows={3}
                  />
                </div>
              </div>
              <div className="modal-footer">
                <button type="button" className="btn-secondary" onClick={closeModal} disabled={isSubmitting}>
                  Cancel
                </button>
                <button type="submit" className="btn-primary" disabled={isSubmitting}>
                  {isSubmitting ? 'Saving...' : 'Save Category'}
                </button>
              </div>
            </form>
          </div>
        </div>
      )}
    </div>
  );
}
