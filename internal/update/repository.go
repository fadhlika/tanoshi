package update

import (
	"gorm.io/gorm"
)

type Repository struct {
	db *gorm.DB
}

func NewRepository(db *gorm.DB) *Repository {
	return &Repository{db}
}

func (r *Repository) GetUpdates() ([]*Update, error) {
	rows, err := r.db.Table("chapters c").
		Joins("JOIN mangas m ON m.id = c.manga_id").
		Where("m.is_favorite = true").
		Order("c.uploaded_at desc").
		Select("m.source, m.id, m.title, m.cover_url, c.id, c.number, c.title, c.uploaded_at").
		Rows()
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var updates []*Update
	for rows.Next() {
		var u Update
		if err := rows.Scan(&u.Source, &u.MangaID, &u.MangaTitle, &u.CoverURL, &u.ChapterID, &u.ChapterNumber, &u.ChapterTitle, &u.UploadedAt); err != nil {
			return nil, err
		}

		updates = append(updates, &u)
	}

	return updates, nil
}