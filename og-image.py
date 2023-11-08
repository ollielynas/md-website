



def add_screenshot(filename):
    hti = Html2Image()
    hti = Html2Image(size=(500, 900), custom_flags=['--virtual-time-budget=10000', '--hide-scrollbars'])
    hti.screenshot(url='https://ollielynas.github.io/md-website/#md_files/home.md', save_as='web_png.png')
