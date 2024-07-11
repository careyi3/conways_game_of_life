# frozen_string_literal: true

def print(print_area, grid)
  print_area.times do |y|
    points = []
    print_area.times do |x|
      points <<
        if grid["#{x}:#{y}"].nil? || grid["#{x}:#{y}"].zero?
          '.'
        else
          '#'
        end
    end
    puts points.join(' ')
  end
end

def neighbour_coords(x, y)
  [
    "#{x + 1}:#{y}",
    "#{x}:#{y + 1}",
    "#{x + 1}:#{y + 1}",
    "#{x - 1}:#{y - 1}",
    "#{x - 1}:#{y}",
    "#{x}:#{y - 1}",
    "#{x + 1}:#{y - 1}",
    "#{x - 1}:#{y + 1}"
  ]
end

print_area = 60
grid = {}
35.times do
  x = rand(print_area / 4..print_area / 2)
  y = rand(print_area / 4..print_area / 2)
  grid["#{x}:#{y}"] = 1
  grid["#{x}:#{y + 1}"] = 1
  grid["#{x + 1}:#{y}"] = 1

  neighbour_coords(x, y).each do |coord|
    grid[coord] = 0 if grid[coord].nil?
  end

  neighbour_coords(x, y + 1).each do |coord|
    grid[coord] = 0 if grid[coord].nil?
  end

  neighbour_coords(x + 1, y).each do |coord|
    grid[coord] = 0 if grid[coord].nil?
  end
end

print(print_area, grid)
sleep(0.1)

generation = 0
loop do
  new_grid = {}
  grid.each do |key, val|
    x, y = key.split(':').map(&:to_i)
    neighbours = []
    neighbour_coords(x, y).each do |coord|
      neighbours << (grid[coord] || 0)
    end
    population = neighbours.select { |s| s == 1 }.count

    if val == 1 && [2, 3].include?(population)
      new_grid["#{x}:#{y}"] = 1
      neighbour_coords(x, y).each do |coord|
        new_grid[coord] = 0 if new_grid[coord].nil?
      end
    end

    next unless val.zero? && population == 3

    new_grid["#{x}:#{y}"] = 1
    neighbour_coords(x, y).each do |coord|
      new_grid[coord] = 0 if new_grid[coord].nil?
    end
  end

  grid = new_grid
  generation += 1
  system('clear')
  print(print_area, new_grid)
  puts "Generation: #{generation}"
  puts "Population: #{new_grid.values.tally[1]}"
  sleep(0.1)
end
