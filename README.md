
    #  ======================================================= #
    #   __  __ _           _       ____  _             _       #
    #  |  \/  (_)_ __   __| |  _  / ___|| |_ __ _  ___| | __   #
    #  | |\/| | | '_ \ / _` | (_) \___ \| __/ _` |/ __| |/ /   #
    #  | |  | | | | | | (_| |  _   ___) | || (_| | (__|   <    #
    #  |_|  |_|_|_| |_|\__,_| (_) |____/ \__\__,_|\___|_|\_\   #
    #                                                          #
    #               A stack of your thoughts/tasks             #
    #                                                          #
    #  ======================================================= #

A bash script to store your thoughts/tasks/whatever on a stack. Usage:

    s [push] some stuff #to add something to the top of stack
    s pop #to pop something from the top of the stack
    s peek #to peek at something on the top of the stack
    s ls #to list the whole stack
    s append #to append a task to the end of the stack
    s reset #to reset the stack
    s edit #to edit the whole list in vi
    s help #to print this usage/help

To sync the stack across your computers. Run the following command:

    mkdir $HOME/Dropbox/mind_stack && ln -nfs $HOME/.mind_stack $HOME/Dropbox/mind_stack

To show the top three tasks in your xmobar, tweak your `.xmobarrc` as per the `.xmobarrc` in the repo

    Config {
           ....
           , commands = [
                        ....
                        , Run Com "/home/minhajuddin/.scripts/s" ["top"] "slotter" 600
                        ....
                        ]
           ....
           , template = ".... <fc=#ffff00>%slotter%</fc> ...."
           }

