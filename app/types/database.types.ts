export type Json
  = | string
    | number
    | boolean
    | null
    | { [key: string]: Json | undefined }
    | Json[]

export interface Database {
  public: {
    Tables: {
      feeds: {
        Row: {
          id: string
          url: string
          name: string
          is_active: boolean
          created_at: string
        }
        Insert: {
          id?: string
          url: string
          name: string
          is_active?: boolean
          created_at?: string
        }
        Update: {
          id?: string
          url?: string
          name?: string
          is_active?: boolean
          created_at?: string
        }
        Relationships: []
      }
      articles: {
        Row: {
          id: string
          feed_id: string | null
          title: string
          url: string
          summary: string | null
          tags: string[] | null
          source: string | null
          published_at: string
          created_at: string
        }
        Insert: {
          id?: string
          feed_id?: string | null
          title: string
          url: string
          summary?: string | null
          tags?: string[] | null
          source?: string | null
          published_at: string
          created_at?: string
        }
        Update: {
          id?: string
          feed_id?: string | null
          title?: string
          url?: string
          summary?: string | null
          tags?: string[] | null
          source?: string | null
          published_at?: string
          created_at?: string
        }
        Relationships: [
          {
            foreignKeyName: 'articles_feed_id_fkey'
            columns: ['feed_id']
            referencedRelation: 'feeds'
            referencedColumns: ['id']
          }
        ]
      }
    }
    Views: {
      [_ in never]: never
    }
    Functions: {
      [_ in never]: never
    }
    Enums: {
      [_ in never]: never
    }
    CompositeTypes: {
      [_ in never]: never
    }
  }
}
