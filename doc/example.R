# install.packages("data.table")
# install.packages("lidR")
# install.packages("Rcpp")
# install.packages("devtools")
# install.packages("TreeLS")
library(HancockBinReader)
library(data.table)
library(lidR)
library(devtools)
library(TreeLS)

folder = "R:/bin_clouds/"
outputPath = "R:/grounds/"
everything = dir(folder, pattern=".*day1_loc1.*", full.names = TRUE)
i = 1
modVal 

for (inFile in everything) {
  if (i % )
  cat(paste0(i, " of ", length(everything), "\n"))
  i = i + 1
  cat(paste0("Reading data..."))
  data = read_hancock_bin(inFile)
  
  cat(paste0("\rComputing xyz from polar data...","                                      "))
  all_beams = data.table(data$beams)
  all_returns = data.table(data$returns)
  rm(data)
  
  # Keep only last hits
  all_returns = all_returns[,list(last_hit=r[.N], r, refl),by = shot_n]
  # Thin up things
  all_returns= all_returns[sample(.N, nrow(all_returns)/3)]
  
  
  setkey(all_beams, shot_n)
  setkey(all_returns, shot_n)
  xyz = all_returns[
    all_beams[,list(shot_n, az, zen, x, y, z),],
    list(
      x=sin(az*pi/180)*r*sin(zen*pi/180)+x,
      y=cos(az*pi/180)*r*sin(zen*pi/180)+y,
      z=cos(zen*pi/180)*r+z,
      refl
    ), nomatch=0]
  rm(all_returns)
  rm(all_beams)
  
  # Convert to LAS
  cat(paste0("\rConverting to LAS...","                                      "))
  names(xyz)=c("X", "Y", "Z", "Intensity")
  xyz$Intensity = integer(nrow(xyz))
  
  las = lidR::LAS(xyz[complete.cases(xyz)])
  
  cat(paste0("\rClassifying ground...","                                      "))
  grounded = lidR::lasground(las, csf())
  
  
  baseFile = basename(inFile)
  outputRaster = gsub(".bin$", "_g.tif", baseFile)
  outputRaster = normalizePath(file.path(outputPath, outputRaster), mustWork = FALSE)
  cat(paste0("\rMaking DTM...","                                      "))
  dtm = grid_terrain(grounded, algorithm = knnidw(k = 6L, p = 2), res=0.2)
  cat(paste0("\rWriting output...","                                      "))
  raster::writeRaster(dtm, outputRaster, overwrite = TRUE)
}

rm(list=ls())
gc()
