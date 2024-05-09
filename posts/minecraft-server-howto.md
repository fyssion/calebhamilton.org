---
title = "How to host a Minecraft server for cheap"
description = "A quick and dirty guide for those who want better, stemming from some past experiences."
created_at = 1708243198
hidden = true
---

Note that before I get into this, I'd like to mention that I haven't hosted a Minecraft server in a few years. I'm mainly writing this for some friends who want to pick up the baton of hosting the yearly Minecraft server. I hosted multiple Minecraft servers per year for around 6 or 7 years, so I'd like to think I have some idea of what I'm doing. Who knows though.

## Getting the machine

If you've ever wanted to host a Minecraft server, and didn't want to host it on your local network, then you've probably looked into some of the game server hosting providers. Maybe you stumbled across server.pro, Apex hosting, or whatever. They're basically all the same. Once upon a time, I used some of these services. Most of the time, they get the job done. Unfortunately, you're almost always paying a prettier penny for a worse experience.

Luckily, there's a better way. A cheaper way, and one that gives you oh so much more control. And who doesn't love a little bit more control? Instead of relying on overpriced, crappy gaming services, let's instead use some *real* hosting providers. With companies like Hetzner, Digital Ocean, and others, you can basically rent an entire computer for as little or as long as you like. Well, usually not an *entire* computer, more like a part of one, but it's basically the same thing, and you're doing that anyway with the gaming-specific ones too. These services like Hetzner rent out things called **virtual private servers** (VPSs). You decide how many vCPUs (virtual CPUs) you want along with the RAM (and storage, although that's pretty irrelevant for reasons I will discuss later), and you basically pay for your own cloud computer by the hour. Yes, that "by the hour" means you do *not* have to commit to purchasing a server for an entire month. So when your friends get bored after two weeks, you can shut the thing down! No money wasted! Also, you are renting an actual computer. It's a full-on computer, not just some dumbed-down gaming server. Like I said, these things are the real deal, used to run a good chunk of the websites you visit. You can even install Windows on there, although I would not recommend doing that.

Anyway, if you're more money-oriented, I'm unfortunately not going to get into the super specific pricing details here, but from what I can gather, Apex charges around $28/mo for 8GB of RAM and who knows how many vCPUs. The equivalent Hetzner server currently runs for a little over $14/mo, and you get 8GB RAM, 4 vCPUs, and 160GB of SSD storage. That means you're saving over $10 per month. That's like half the cost of a streaming service in 2024! Fuck streaming services. Anyway, if you need more storage, you can rent additional storage volumes and add many more GB. Though you probably won't ever get to that point, especially if you're reading this guide. Also, Hetzner offers a bunch of other options, ranging from around $4/mo (probably not good enough for a Minecraft server) to a lot more if you want many gigs of RAM. As an aside, please (please) note that these prices were captured in February 2024, so your milage may vary if you are reading this in the future.

If you want my advice for which VPS in particular to rent, I'd personally recommend a shared vCPU VPS from Hetzner running Ubuntu 22.04. I probably wouldn't go lower than 4GB of RAM, and I usually go with 8GB myself. If you need more CPU power, try renting a dedicated server instead of a shared one. They're a bit pricier, but that way you get the full machine instead of just a virtual part of it. The rest of this guide assumes you've rented an Ubuntu VPS, so make sure you do that with whatever provider you go with.

## Setting things up

Okay so we've got a cloud computer to run the Minecraft server on. How do we actually set up the thing? Well, the beauty of renting our own computer is we've got options! Want the simplest possible solution? Just download a Minecraft server JAR from minecraft.net and run it. I don't actually recommend this solution, as it'll end up being a lot more painful to maintain. Instead, I'm going to walk you through setting up a server management web panel like the ones the gaming hosting services provide. You can set one up in less than thirty minutes.

In this guide, we're going to use an open source project called [PufferPanel](https://www.pufferpanel.com/). It's not the best web panel, nor is it the flashiest, but it gets the job done, and more importantly, it's incredibly easy to install and set up. You can even do this in a few minutes with enough skill (I have!).

First thing's first, we need to do some housekeeping. In whatever hosting service you chose, open a console to the server and log in. Yes, you will have to enter a few commands in a console. Don't worry, I'll try to make it as painless as possible. After running the commands in this guide, you won't have to touch the console again.

If you rented a VPS with Hetzner, you'll have to check your email for the root login password. Once you enter it, it'll make you set a new password.

Okay, here are the first few commands we'll need to enter. Instead of doing everything through the root user, which is basically a super-admin user, we'll instead want to create our own account. This is mainly for security reasons. To create a new account, enter the command below, replacing `<username>` with your desired username of the account:

```sh
adduser <username>
```

After asking for a password, it'll ask you for a bunch of information relating to the account. You can skip all of this by just spamming the enter key until it's finished.

Now, we need to grant our new user admin privileges. Run the following command to do this, again replacing `<username>` with the username you chose above:

```sh
usermod -aG sudo <username>
```

Account created! To switch to your new account without logging out and back in again, you can use this (again `<username>` is the same as in the above steps):

```sh
su <username>
```

## Installing the panel

Okay, you're a good chunk of the way there! Next, we're going to install PufferPanel, our web panel of choice, to manage our Minecraft server.

To do this, simply run the commands listed below, one after the other. Note that these commands were taken from the [PufferPanel installation guide][pufferpanel-guide], so if they don't work, check there for the most recent commands.

```sh
curl -s https://packagecloud.io/install/repositories/pufferpanel/pufferpanel/script.deb.sh | sudo bash

sudo apt-get install pufferpanel

sudo systemctl enable pufferpanel
```

Next, let's create the user account that you'll use to login to the web portal. Run this command and answer its questions.

```sh
sudo pufferpanel user add
```

Finally, we can start PufferPanel:

```sh
sudo systemctl start pufferpanel
```

In the next steps, you'll need the IP address for the VPS you're using. If you don't know how to get this, the following command will tell you:

```sh
curl ifconfig.co
```

## Creating the Minecraft server

Alright, we've pretty much run all the commands we'll ever needs to run! Congrats, you've done the hard part. You should now be able to perform the rest of the setup comfortably within a web browser. Go ahead and open one up, grab the IP address of your server from the step above, and enter in `ip-address:8080` to the address bar (replacing `ip-address` with your VPS's IP address of course). You should be able to enter the credentials you described in the steps above to gain access to the panel.

Inside PufferPanel, the first thing we need to do is import the Minecraft server template. Navigate to the `Templates` page on the left navigation bar, and when it prompts you to import a template, select one of the `minecraft-xxx` templates. I usually ran a Spigot or Paper server, but if you don't know what those are, just choose `minecraft-vanilla`.

Almost there! Now just head to the `Servers` page (again on the left), and create a new server. You'll need to choose a name for it. You can leave all the other fields untouched, and just keep pressing `Next` until you get to the `Options` page. Make sure to tick the "EULA Agreement" checkbox. You'll also want to set the memory limit for the Minecraft server, located in the "Memory (MB)" text box. Generally, this should be a gigabyte or two less than your VPS's physical memory to give room for Java's JVM. This value must be in the binary representation of megabytes. If you are unsure of what that means, start with the number 1024 and multiply it by the number of gigabytes you want to give your Minecraft server. For example, if I had an 8GB VPS and wanted to give my server 5GB, I would multiply `1024 * 5 = 5120`, and enter 5120 into PufferPanel. Feel free to change any of the other settings if you know what they mean, then click the `Create` button at the bottom.

Once that's done, simply click the `Install` button in the upper right, then click the `Start` button. The first time your server starts, it might say something about subscribing to an EULA. If this happens, don't panic! This is expected. All you need to do is navigate to the `Files` tab on the bottom of the screen, find and click on the file called `eula.txt`, and set the contents to `eula=true`. You should now be able to start your server again (or click `Kill` and then `Start` if it's showing a `Restart` button instead).

## Connecting

And that's it! You should now have a fully-functional Minecraft server up and running. To connect to this server, just paste the IP address into your Minecraft client in the Multiplayer page when adding a new server.

## Additional tips

Once your Minecraft server is up, you may want to change some additional settings. If you you're already familiar with the `server.properties` file, you can skip this part. Otherwise, I'm going to recommend some settings to change. Go ahead and navigate to the `Files` tab near the bottom of the screen in PufferPanel, and select the file named `server.properties`.

Reading the `server.properties` file is pretty simple. Each line is formatted as `property=value`, where `property` is the name of the setting and `value` is its value. If you skim the file, you'll see settings like `gamemode=survival`. This means the gamemode of the server is set to survival mode. Speaking of the gamemode, if you'd like to change it, find the `gamemode` property and set it to one of `survival`, `creative`, `adventure`, or `spectator`. For example, to set the gamemode to creative, you'd write `gamemode=creative`. If you want your server to be in hardcore mode, there's a property called `hardcore` that you can set to `true`. You can also set the difficulty using the `difficulty` property to any one of `peaceful`, `easy`, `normal` and `hard`.

If you want to change the seed of your Minecraft server, there's a property called `level-seed`. Note that if you've already started your server once before, you'll need to either delete the old world files or set the `level-name` property to a different name in order to regenerate the world.

Finally, there are a few other properties you'd probably like to set. The `spawn-protection` property prevents anyone from breaking/placing blocks in a certain radius around spawn. You should probably set this to `0` unless you want that feature. The `view-distance` controls how far players can see. This is similar to the render distance setting in your Minecraft client, except it applies to everyone in the server. Note that the higher you set this, the more resources your server will use. You can also increase/decrease the max player count with the `max-players` property. For more details about `server.properties`, check out the [Minecraft Wiki page about it][wiki-server-properties].

After modifying `server.properties`, you'll need to restart the Minecraft server for the changes to take effect.

If you want to make yourself an admin/operator of the Minecraft server, you can run the `op <username>` command in the PufferPanel console, where `<username>` is your Minecraft username.

## Wrapping up

Congratulations for making it all the way through this guide. The best part about this setup is you can use PufferPanel to run almost any game server imaginable. I recently helped some friends set up a Palworld server using a similar process to this guide. PufferPanel offers a plethora of game templates to choose from. You won't have to worry about renting from another crappy hosting provider again!

Anyway, thanks for reading my guide. If you have any questions, got stuck somewhere, or just want to reach out, my email is `me@` this domain.

[pufferpanel-guide]: https://docs.pufferpanel.com/en/latest/installing.html
[wiki-server-properties]: https://minecraft.fandom.com/wiki/Server.properties
