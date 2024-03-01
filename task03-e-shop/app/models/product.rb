class Product < ApplicationRecord
  has_many :cart_items

  validates :name, presence: true
  validates :description, presence: true
  validates :price, presence: true
  validates :image, presence: true

  has_one_attached :image
end
