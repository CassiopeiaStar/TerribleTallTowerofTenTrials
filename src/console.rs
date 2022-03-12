
use macroquad::prelude::*;

const SELECTION_KEYS: [char;22] = ['a','b','c','d','e','f','g','i','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];

#[derive(Debug,Default,Copy,Clone)]
pub struct Cell {
    pub fg: Option<Color>,
    pub bg: Option<Color>,
    pub c: Option<u32>,
    pub layer: u32,
    pub ignore_overlap: bool,
}

impl Cell {
    pub fn set_fg(&mut self, color: Color) -> &mut Self {
        self.fg.replace(color);
        self
    }

    pub fn set_bg(&mut self, color: Color) -> &mut Self {
        self.bg.replace(color);
        self
    }

    pub fn set_layer(&mut self, layer: u32) -> &mut Self {
        self.layer = layer;
        self
    }

    pub fn set_ignore_overlap(&mut self, ignore_overlap: bool) -> &mut Self {
        self.ignore_overlap = ignore_overlap;
        self
    }
    pub fn set_c(&mut self, c: u32) -> &mut Self {
        self.c.replace(c);
        self
    }
}

pub struct AsciiConsole {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
}

impl AsciiConsole {
    pub fn new(width: usize, height: usize, default_cell: Option<Cell>) -> Self {
        let default_cell = default_cell.unwrap_or(Cell {
            fg: None,
            bg: None,
            c: Some(' ' as u32),
            layer: 0,
            ignore_overlap: true,
        });
        Self {
            cells: vec![default_cell;width*height],
            width,height,
        }
    }

    pub fn get(&self,pos:&(i32,i32)) -> Option<&Cell> {
        let (x,y) = pos;
        if *x > -1+ self.width as i32 || *y > -1+self.height as i32 
         || *x < 0 || *y < 0 {
            return None;
        }
        self.cells.get(*x as usize + (*y as usize * self.width))
    }

    pub fn get_mut(&mut self,pos:&(i32,i32)) -> Option<&mut Cell> {
        let (x,y) = pos;
        if *x > -1+self.width as i32 || *y > -1+self.height as i32
         || *x < 0 || *y < 0 {
            return None;
        }
        self.cells.get_mut(*x as usize + (*y as usize * self.width))
    }

    pub fn set(&mut self, pos:&(i32,i32), new_cell:Cell) {
        if let Some(cell) = self.get_mut(pos) {
            *cell = new_cell;
        }
    }

    pub fn set_fg(&mut self, pos:&(i32,i32), color: Color) -> &mut Self {
        if let Some(cell) = self.get_mut(pos) {
            cell.set_fg(color);
        }
        self
    }

    pub fn set_bg(&mut self, pos:&(i32,i32), color: Color) -> &mut Self {
        if let Some(cell) = self.get_mut(pos) {
            cell.set_bg(color);
        }
        self
    }

    pub fn set_layer(&mut self, pos:&(i32,i32), layer: u32) -> &mut Self {
        if let Some(cell) = self.get_mut(pos) {
            cell.set_layer(layer);
        }
        self
    }

    pub fn set_c(&mut self, pos:&(i32,i32), c: u32) -> &mut Self {
        if let Some(cell) = self.get_mut(pos) {
            cell.set_c(c);
        }
        self
    }

    pub fn blit(&self, pos:&(i32,i32), dest: &mut AsciiConsole) {
        for x in 0..self.width as i32{
            for y in 0..self.height as i32{
                let source_pos = &(x,y);
                let dest_pos = &(x+pos.0,y+pos.1);
                if let Some(source_cell) = self.get(source_pos) {
                    if let Some(dest_cell) = dest.get_mut(dest_pos) {
                        *dest_cell = source_cell.clone();
                    }
                }
            }
        }
    }

    pub fn rectangle(&mut self,fg:Color,bg:Color,x:i32,y:i32,w:i32,h:i32) {
        let w = w-1;
        let h = h-1;
        for i in 1..w {
            self.set_c(&(x+i,y),196);
            self.set_fg(&(x+i,y),fg);
            self.set_bg(&(x+i,y),bg);
            self.set_c(&(x+i,y+h),196);
            self.set_fg(&(x+i,y+h),fg);
            self.set_bg(&(x+i,y+h),bg);
        }
        for j in 1..h {
            self.set_c(&(x,y+j),179);
            self.set_fg(&(x,y+j),fg);
            self.set_bg(&(x,y+j),bg);
            self.set_c(&(x+w,y+j),179);
            self.set_fg(&(x+w,y+j),fg);
            self.set_bg(&(x+w,y+j),bg);
        }
        self.set_c(&(x,y),218);
        self.set_fg(&(x,y),fg);
        self.set_bg(&(x,y),bg);
        self.set_c(&(x,y+h),192);
        self.set_fg(&(x,y+h),fg);
        self.set_bg(&(x,y+h),bg);
        self.set_c(&(x+w,y),191);
        self.set_fg(&(x+w,y),fg);
        self.set_bg(&(x+w,y),bg);
        self.set_c(&(x+w,y+h),217);
        self.set_fg(&(x+w,y+h),fg);
        self.set_bg(&(x+w,y+h),bg);
    }

    pub fn progress_bar(&mut self,color:Color,x:i32,y:i32,w:i32,ratio:f32) {
        let fill = (ratio* (w as f32 -2.)) as i32;
        self.set_c(&(x,y),'|' as u32);
        self.set_fg(&(x,y),WHITE);
        self.set_c(&(x+w-1,y),'|' as u32);
        self.set_fg(&(x+w-1,y),WHITE);
        for i in 0..w-2 {
            if i < fill {
                self.set_c(&(x+1+i,y),'#' as u32);
                self.set_fg(&(x+1+i,y),color);
            } else {
                self.set_c(&(x+1+i,y),'-' as u32);
                self.set_fg(&(x+1+i,y),DARKGRAY);
            }
        }
    }

    pub fn print_line(&mut self,fg:Color,bg:Color,x:i32,y:i32,text: String) {
        for (i,c) in text.char_indices() {
            self.set_fg(&(x+i as i32,y),fg);
            self.set_bg(&(x+i as i32,y),bg);
            self.set_c (&(x+i as i32,y),c as u32);
        }
    }

    pub fn print_multi_line(&mut self,fg:Color,bg:Color,x:i32,y:i32,width:i32,text: String) -> i32 {
        let mut lines = Vec::new();
        let mut remaining_text = text;
        while remaining_text.len() as i32 > width {
            let mut last_space = None;
            for (i,c) in remaining_text.char_indices() {
                if c == ' ' {
                    last_space.replace(i);
                }
                if i as i32 > width {break;}
            }
            let last_space = last_space.unwrap_or(width as usize);
            let rest = remaining_text.split_off(last_space+1);
            lines.push(remaining_text);
            
            remaining_text = rest;
        }
        lines.push(remaining_text);

        for (i,line) in lines.iter().enumerate() {
            self.print_line(fg,bg,x,y+i as i32,line.clone());
        }
        
        return lines.len() as i32;
    }

    pub fn draw(
        &self, 
        sprite_sheet: &Texture2D,
        sprite_size: &(f32,f32),
        sprite_sheet_columns: u32,
        cell_draw_size: &(f32,f32),
        position_on_screen: &(f32,f32),
    ) {
        for x in 0..self.width {
            for y in 0..self.height {
                let cell = self.get(&(x as i32,y as i32));
                if let Some(cell) = cell {
                    //if a background is stored, draw a rectangle
                    if let Some(bg) = cell.bg {
                        draw_rectangle(
                            position_on_screen.0 + (cell_draw_size.0 * x as f32),
                            position_on_screen.1 + (cell_draw_size.1 * y as f32),
                            cell_draw_size.0,
                            cell_draw_size.1,
                            bg
                        );
                    }

                    if let (Some(fg),Some(c)) = (cell.fg,cell.c) {
                        let ascii_x = c % sprite_sheet_columns;
                        let ascii_y = c / sprite_sheet_columns;
                        let macroquad_draw_params = DrawTextureParams {
                            dest_size: Some(vec2(cell_draw_size.0,cell_draw_size.1)),
                            source: Some(Rect::new(
                                    sprite_size.0 * ascii_x as f32,
                                    sprite_size.1 * ascii_y as f32,
                                    sprite_size.0,
                                    sprite_size.1
                            )),
                            ..Default::default()
                        };
                        draw_texture_ex(
                            *sprite_sheet,
                            position_on_screen.0 + (cell_draw_size.0 * x as f32),
                            position_on_screen.1 + (cell_draw_size.1 * y as f32),
                            fg,
                            macroquad_draw_params
                        );
                    }
                }
            }
        }
    }
}


pub struct ConsoleMenu {
    name: Option<String>,
    size: (usize,usize),
    selection: usize,
    fg: Color,
    bg: Color,
    options: Vec<String>,
}
impl ConsoleMenu {
    pub fn new(name: Option<String>, size: (usize,usize), fg: Color,bg: Color, options: Vec<String>) -> Self {
        while get_char_pressed().is_some() {};

        Self {
            name,
            size,
            selection: 0,
            options,
            fg,
            bg,
        }
    }

    pub fn check_selection(&mut self) -> Option<usize> {
        if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space) {
            return Some(self.selection);
        }

        if is_key_pressed(KeyCode::J) || 
            is_key_pressed(KeyCode::Down) || 
            is_key_pressed(KeyCode::Kp2) {
                if self.selection < self.options.len()-1 {
                    self.selection += 1;
                }
        }
        if is_key_pressed(KeyCode::K) || 
            is_key_pressed(KeyCode::Up) || 
            is_key_pressed(KeyCode::Kp8) {
                if self.selection > 0 {
                    self.selection -= 1;
                }
        }

        if let Some(c) = get_char_pressed() {
            if let Some(selection) = SELECTION_KEYS.iter().position(|x| {*x==c}) {
                if selection < self.options.len() {
                    return Some(selection);
                }
            }
        }

        None
    }

    pub fn console(&self) -> AsciiConsole {
        let mut default_cell = Cell::default();
        default_cell.set_fg(self.fg).set_bg(self.bg);
        let mut con = AsciiConsole::new(self.size.0,self.size.1,Some(default_cell));
        
        con.rectangle(self.fg,self.bg,0,0,self.size.0 as i32,self.size.1 as i32);
        if let Some(name) = self.name.clone() {
            con.print_line(self.fg,self.bg,1,0,name);
        }

        let max_options = SELECTION_KEYS.len();
        for (i,option) in self.options.iter().enumerate() {
            if i >= max_options {break;}
            let y = i as i32+2;
            let key = SELECTION_KEYS[i];
            let string = format!("{}: {}",key,option);
            let (fg,bg) = if i == self.selection {
                (self.bg,self.fg)
            } else {
                (self.fg,self.bg)
            };
            con.print_line(fg,bg,2,y,string);
        }
        
        con
    }
}
