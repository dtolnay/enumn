import random

def generate_maze(width, height):
    maze = [[0 for _ in range(width)] for _ in range(height)]
    for i in range(height):
        for j in range(width):
            if random.random() < 0.3:  # 30% 的概率生成墙
                maze[i][j] = 1
    maze[0][0] = 0  # 起点
    maze[height - 1][width - 1] = 0  # 终点
    return maze

def print_maze(maze, player_pos):
    for i, row in enumerate(maze):
        for j, cell in enumerate(row):
            if (i, j) == player_pos:
                print("P", end=" ")  # 玩家位置
            elif cell == 1:
                print("#", end=" ")  # 墙
            else:
                print(".", end=" ")  # 空地
        print()

def move_player(maze, player_pos, move):
    new_pos = player_pos
    if move == "w" and player_pos[0] > 0 and maze[player_pos[0] - 1][player_pos[1]] != 1:
        new_pos = (player_pos[0] - 1, player_pos[1])
    elif move == "s" and player_pos[0] < len(maze) - 1 and maze[player_pos[0] + 1][player_pos[1]] != 1:
        new_pos = (player_pos[0] + 1, player_pos[1])
    elif move == "a" and player_pos[1] > 0 and maze[player_pos[0]][player_pos[1] - 1] != 1:
        new_pos = (player_pos[0], player_pos[1] - 1)
    elif move == "d" and player_pos[1] < len(maze[0]) - 1 and maze[player_pos[0]][player_pos[1] + 1] != 1:
        new_pos = (player_pos[0], player_pos[1] + 1)
    return new_pos

def main():
    width, height = 10, 10
    maze = generate_maze(width, height)
    player_pos = (0, 0)

    while player_pos != (height - 1, width - 1):
        print_maze(maze, player_pos)
        move = input("Enter your move (w/a/s/d): ")
        player_pos = move_player(maze, player_pos, move)

    print("Congratulations! You've escaped the maze!")

if __name__ == "__main__":
    main()
