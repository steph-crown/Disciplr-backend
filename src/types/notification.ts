export type NotificationData = Record<string, unknown> | null

export interface Notification {
  id: string
  user_id: string
  type: string
  title: string
  message: string
  data: NotificationData
  idempotency_key: string | null
  read_at: string | null
  archived_at: string | null
  created_at: string
}

export interface CreateNotificationInput {
  user_id: string
  type: string
  title: string
  message: string
  data?: NotificationData
  idempotency_key?: string
}

export type NotificationSortField = 'created_at' | 'read_at' | 'title' | 'type'
export type NotificationReadStatus = 'all' | 'read' | 'unread'

export interface NotificationListOptions {
  cursor?: string
  limit: number
  includeArchived?: boolean
  readStatus?: NotificationReadStatus
}

export interface NotificationListResult {
  data: Notification[]
  pagination: {
    limit: number
    cursor: string | null
    next_cursor?: string
    has_more: boolean
    count: number
  }
}
