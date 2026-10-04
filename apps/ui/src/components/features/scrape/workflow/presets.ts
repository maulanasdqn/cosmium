import type { TWorkflowStep } from './types'

export type TWorkflowPreset = {
  id: string
  label: string
  description: string
  steps: TWorkflowStep[]
}

export const workflowPresets: TWorkflowPreset[] = [
  {
    id: 'product-listing',
    label: 'Product listing',
    description: 'Scroll to load items, then pull titles, prices, and links',
    steps: [
      { type: 'scroll', infinite: false, selector: null, times: 3 },
      { type: 'delay', duration_ms: 1000 },
      { type: 'extract', name: 'titles', selector: '.product-title', attribute: null, limit: 0 },
      { type: 'extract', name: 'prices', selector: '.price', attribute: null, limit: 0 },
      { type: 'extract', name: 'links', selector: 'a.product-link', attribute: 'href', limit: 0 },
    ],
  },
  {
    id: 'paginated-list',
    label: 'Paginated list',
    description: 'Take the first page, click next, then take the second',
    steps: [
      { type: 'extract', name: 'page_1', selector: '.item', attribute: null, limit: 0 },
      { type: 'click', selector: 'a[rel=next]' },
      { type: 'delay', duration_ms: 2000 },
      { type: 'extract', name: 'page_2', selector: '.item', attribute: null, limit: 0 },
    ],
  },
  {
    id: 'follow-detail-pages',
    label: 'Follow detail pages',
    description: 'Open each result and collect fields from the detail page',
    steps: [
      { type: 'scroll', infinite: false, selector: null, times: 2 },
      {
        type: 'follow_urls',
        name: 'details',
        selector: 'a.result-link',
        attribute: 'href',
        limit: 5,
        workflow: [
          { type: 'delay', duration_ms: 800 },
          { type: 'extract', name: 'title', selector: 'h1', attribute: null, limit: 1 },
          { type: 'extract', name: 'price', selector: '.price', attribute: null, limit: 1 },
        ],
      },
    ],
  },
]
