import json

class Replay():
    def getData(path:str):
        with open(path, "r") as file:
            data = json.load(file)
            return data
    def getName(path:str):
        data = Replay.getData(path)
        return data["properties"]["ReplayName"]
    def getDate(path:str):
        data = Replay.getData(path)
        date = data["properties"]["Date"].split(" ")
        dateDate = date[0].replace("-", ".")
        dateTime = date[1].replace("-", ":")
        return [dateDate, dateTime]
    def getTeamScore(path:str, team:int):
        data = Replay.getData(path)
        teamscore = 0
        for player in data["properties"]["PlayerStats"]:
            if player["Team"] == team:
                teamscore += player["Goals"]
        return teamscore
    def getAll(path:str):
        print(Replay.getName(path))
        print(Replay.getDate(path))
        print("Team 0: " + str(Replay.getTeamScore(path, 0)))
        print("Team 1: " + str(Replay.getTeamScore(path, 1)))

Replay.getAll(str(input("Name of the JSON output file: ")))
